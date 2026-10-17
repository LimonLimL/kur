use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use ohkami::Response;
use ohkami::claw::{Json, Path};
use ohkami::fang::Context;
use ohkami::serde::{Deserialize, Serialize};
use time::UtcDateTime;
use uuid::Uuid;

use crate::contracts::item_storage::ItemStorage;
use crate::contracts_impl::item_sqlite_storage::ItemSqliteStorage;
use crate::contracts_impl::managed_connection_pool::ManagedConnectionPool;
use crate::models::item::Item;

/// Общий контекст запроса: пул соединений с базой данных.
///
/// Пул хранится в `Arc`, поэтому при клонировании контекста для каждого
/// запроса копируется только указатель на общий пул, а не сам пул.
#[derive(Clone)]
pub struct RequestContext {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl RequestContext {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}

/// Модель входных данных для создания товара.
#[derive(Deserialize)]
pub struct AddItemPayload {
    pub name: String,
    pub company: String,
    pub model: String,
    #[serde(default)]
    pub characteristics: HashMap<String, String>,
    pub price: f32,
    pub guarantee_period: i32,
    pub image: String,
}

/// Модель входных данных для обновления товара.
#[derive(Deserialize)]
pub struct UpdateItemPayload {
    pub name: String,
    pub company: String,
    pub model: String,
    #[serde(default)]
    pub characteristics: HashMap<String, String>,
    pub price: f32,
    pub guarantee_period: i32,
    pub image: String,
}

/// Модель ответа: товар в том виде, в каком его видит клиент.
#[derive(Serialize)]
pub struct ItemResponse {
    id: String,
    name: String,
    company: String,
    model: String,
    characteristics: HashMap<String, String>,
    price: f32,
    guarantee_period: i32,
    image: String,
}

impl ItemResponse {
    fn from_item(item: &Item) -> Self {
        Self {
            id: item.id.to_string(),
            name: item.name.clone(),
            company: item.company.clone(),
            model: item.model.clone(),
            characteristics: item.characteristics.clone(),
            price: item.price,
            guarantee_period: item.guarantee_period,
            image: item.image.clone(),
        }
    }
}

/// Единый формат ответа при ошибке.
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

/// Ответ `500 Internal Server Error` с записью подробностей в лог сервера.
fn database_error(message: String) -> Response {
    eprintln!("Ошибка базы данных: {message}");
    Response::InternalServerError().with_json(ErrorResponse {
        message: "database error".to_string(),
    })
}

/// Выполняет операцию с хранилищем товаров на выделенном соединении из пула.
///
/// Соединение берётся из пула перед операцией и возвращается обратно после
/// её завершения — даже если операция закончилась ошибкой.
async fn with_item_storage<T, F, Fut>(context: &RequestContext, operation: F) -> Result<T, String>
where
    F: FnOnce(ItemSqliteStorage) -> Fut,
    Fut: Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    let client = context
        .connection_pool
        .acquire_connection()
        .await
        .map_err(|error| error.to_string())?;

    // Клон `Client` указывает на то же самое соединение, поэтому все
    // запросы внутри одной операции идут через выделенное соединение.
    let storage = ItemSqliteStorage::new(client.clone());
    let result = operation(storage).await.map_err(|error| error.to_string());

    if let Err(error) = context.connection_pool.return_connection(client).await {
        eprintln!("Не удалось вернуть соединение в пул: {error}");
    }

    result
}

/// `POST /add-item` — добавление нового товара на склад.
pub async fn add_item(
    Json(payload): Json<AddItemPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let item = Item::create_new(
        payload.name,
        payload.company,
        payload.model,
        payload.characteristics,
        payload.price,
        payload.guarantee_period,
        payload.image,
    );
    // Ответ готовится до вызова `add`, потому что `add` забирает `item` во владение.
    let response_body = ItemResponse::from_item(&item);

    match with_item_storage(
        &context,
        move |storage| async move { storage.add(item).await },
    )
    .await
    {
        Ok(()) => Response::Created().with_json(response_body),
        Err(message) => database_error(message),
    }
}

/// `GET /items/:id` — получение товара по идентификатору.
pub async fn get_item(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid item id");
    };

    match with_item_storage(
        &context,
        move |storage| async move { storage.get(id).await },
    )
    .await
    {
        Ok(Some(item)) => Response::OK().with_json(ItemResponse::from_item(&item)),
        Ok(None) => Response::NotFound().with_text("item not found"),
        Err(message) => database_error(message),
    }
}

/// `PUT /items/:id` — изменение существующего товара.
pub async fn update_item(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateItemPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid item id");
    };

    // Чтение и запись выполняются на одном выделенном соединении:
    // это и есть смысл управляемого пула соединений.
    let result = with_item_storage(&context, move |storage| async move {
        let mut item = match storage.get(id).await? {
            Some(item) => item,
            None => return Ok(None),
        };

        item.name = payload.name;
        item.company = payload.company;
        item.model = payload.model;
        item.characteristics = payload.characteristics;
        item.price = payload.price;
        item.guarantee_period = payload.guarantee_period;
        item.image = payload.image;
        item.updated_at = Some(UtcDateTime::now());

        storage.update(&item).await?;

        Ok(Some(item))
    })
    .await;

    match result {
        Ok(Some(item)) => Response::OK().with_json(ItemResponse::from_item(&item)),
        Ok(None) => Response::NotFound().with_text("item not found"),
        Err(message) => database_error(message),
    }
}

/// `DELETE /items/:id` — удаление товара со склада.
pub async fn remove_item(
    Path(id): Path<&str>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid item id");
    };

    match with_item_storage(
        &context,
        move |storage| async move { storage.remove(id).await },
    )
    .await
    {
        Ok(()) => Response::OK().with_text("item removed"),
        Err(message) => database_error(message),
    }
}
