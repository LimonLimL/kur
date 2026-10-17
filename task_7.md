# Доработка функционала управления продуктами

Вернемся к http-обработчикам:

```rust
use ohkami::{Response, Status};

pub async fn add_item() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("added item");
    return response;
}

pub async fn remove_item() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("removed item");
    return response;
}

pub async fn update_item() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated item");
    return response;
}

pub async fn get_item() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("got item");
    return response;
}
```

Сейчас они только возвращают текст: товар не создаётся, не изменяется и не удаляется. Сначала научимся принимать данные запроса, затем подключим адаптер для работы с БД в обработчики.

## `POST` запросы.

Для создания товара отправим `POST /add-item` с названием и ценой.

### Тело запроса и JSON

Тело запроса (request body) — данные, отправляемые после заголовков HTTP-запроса. Мы передаём их в формате JSON, например `{"name":"Карандаш","price":25.5}`. В отличие от адреса запроса, тело удобно для нескольких полей. У сущности `Item` есть также идентификатор и даты, но будет создавать сервер:

```rust
#[doc = "Сущность товар:
        Поля сущности:
        - идентификатор
        - название товара
        - цена товара
        - дата создания
        - дата удаления
        - дата обновления"]
pub struct Item {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) price: f32,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}
```

Поэтому модель входных данных содержит только два поля:

```rust
pub struct AddItemPayload {
    pub name: String,
    pub price: f32,
}
```

Если просто передать структуру как параметр `add_item(payload: AddItemPayload)`, Ohkami не поймёт, из какой части HTTP-запроса её получить: нужен тип-извлекатель `Json<AddItemPayload>`. Без него при регистрации маршрута возникнет ошибка о том, что функция не может быть обработчиком (`IntoHandler`).

Чтобы преобразовать JSON в нашу структуру, добавим `serde` с поддержкой `derive` в список библиотек `Cargo.toml` (если она ещё не подключена):

```toml
[dependencies.serde]
version = "1.0"
features = ["derive"]

[dependencies.serde_json]
version = "1.0"
```

Использовать библиотеку `serde` для десериализации JSON просто. Использование макроса `Deserialize` учит `serde` заполнять поля структуры из JSON:

```rust
use ohkami::{Json, Response, Status};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddItemPayload {
    pub name: String,
    pub price: f32,
}

pub async fn add_item(Json(payload): Json<AddItemPayload>) -> Response {
    println!("{}: {}", payload.name, payload.price);
    let mut response = Response::new(Status::OK);
    response.set_text("added item");
    response
}
```

`Json(payload)` извлекает из обёртки данные типа `AddItemPayload`. Заголовок `Content-Type: application/json` сообщает серверу формат тела. Проверка с запущенным сервером:

```shell
curl -i -X POST http://localhost:3000/add-item -H 'Content-Type: application/json' -d '{"name":"Карандаш","price":25.5}'
```

Такой обработчик пока только печатает данные и возвращает `200 OK`. `Item::create_new(payload.name, payload.price)` создаёт сущность с ID и датой, **но не сохраняет её**. За сохранение отвечает `ItemsSqliteStorage::add`.

### Контекст приложения и соединение

Обработчику `add_item` понадобятся не только `name` и `price` из запроса, но и доступ к БД. Если создавать подключение внутри каждого обработчика, разные обработчики не будут использовать общий пул. Контекст запроса в Ohkami позволяет при настройке сервера передать общие зависимости обработчикам: например, пул соединений.

В `main` мы один раз создаём пул и регистрируем `Context::new(request_context)` вместе с маршрутами. Когда приходит запрос, Ohkami делает этот контекст доступным соответствующему обработчику. Обработчик указывает, что ему нужен контекст, в параметрах:

```rust
use ohkami::{Json, Response, fang::Context};

pub async fn add_item(
    Json(payload): Json<AddItemPayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    println!("Adding item: {}", payload.name);
    let _pool = &context.connection_pool;
    Response::NotImplemented()
}
```

`Context(context)` содержит ссылку на `RequestContext`, поэтому через `context.connection_pool` обработчик сможет запросить соединение. `ItemsSqliteStorage` ожидает `async_sqlite::Client`, который позже можно будет взять из этого пула. Сам контекст не создаёт соединение и не выполняет запрос к БД.

Сначала попробуем хранить пул непосредственно в контексте:

```rust
pub struct RequestContext {
    connection_pool: ManagedConnectionPool,
}

// В main после создания пула:
let request_context = RequestContext { connection_pool: pool };
Context::new(request_context);
```

При вызове `Context::new` компилятор сообщит, что `RequestContext` не реализует `Clone` (`the trait bound RequestContext: Clone is not satisfied`). Ohkami требует клонируемые данные для контекста каждого запроса. Просто добавить `#[derive(Clone)]` здесь недостаточно: тогда `Clone` потребуется и полю `ManagedConnectionPool`, у которого его нет. Переданный в структуру `pool` также перемещается в неё: повторно использовать эту переменную нельзя.

Важно различать регистрацию и использование контекста. `Context::new(request_context)` получает значение при настройке сервера, а параметр обработчика `Context(context): Context<'_, RequestContext>` получает **ссылку** на доступный в запросе контекст. Ошибка возникает не потому, что обработчик пытается забрать пул из этой ссылки, а из-за требования `Clone` к типу, передаваемому в `Context::new`.

Обернём пул в `Arc`: теперь при клонировании `RequestContext` будет клонироваться указатель на общий пул, а не сам пул.

```rust
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestContext {
    connection_pool: Arc<ManagedConnectionPool>,
}

impl RequestContext {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}
```

`Arc` позволяет нескольким контекстам владеть ссылкой на один пул; он не клонирует сами соединения и сам по себе не делает операции с БД транзакционными.

В `main` создадим пул и зарегистрируем контекст перед маршрутами:

```rust
use std::sync::Arc;
use ohkami::{Ohkami, Route, fang::Context};

// Фрагмент внутри асинхронного блока main (нужны доступные типы и модули):
let pool = ManagedConnectionPool::initialize("database.db".into(), 10).await;
let request_context = RequestContext::new(Arc::new(pool));
Ohkami::new((
    Context::new(request_context),
    "/add-item".POST(http::add_item),
    "/items/:id".GET(http::get_item).PUT(http::update_item).DELETE(http::remove_item),
))
.howl("localhost:3000")
.await;
```

Здесь показан **целевой** набор маршрутов; он не заменяет автоматически текущие маршруты проекта. Путь `database.db` сам по себе не создаёт таблицу `items`: миграция базы должна быть выполнена отдельно.

Для сохранения товара обработчику понадобится взять `Client` через `acquire_connection().await`, вызвать `ItemsStorage::add(item).await` и вернуть клиент через `return_connection(...)`. 

Но текущий конструктор `ItemsSqliteStorage::new(client).await` **забирает** `Client` во владение и не отдаёт его обратно. Поэтому такой порядок пока нельзя реализовать буквально: сначала требуется изменить API адаптера или способ управления клиентом. 

`Arc` проблему возврата не решает. Если операция закончится с ошибкой до возврата, соединение тоже должно вернуться в пул; для этого нужна отдельная гарантия возврата. Следующие методы показывают HTTP-слой, пока без реализации этого шага работы с БД.

## `GET` запросы

`GET` нужен для чтения товара. В отличие от `POST`, для выбора одного товара обычно передают его идентификатор в адресе: `GET /items/550e8400-e29b-41d4-a716-446655440000`. Для списка товаров можно использовать `GET /items`. Сам факт вызова `GET` просто говорит о том, что, клиент хочет получить данные о товаре или списке товаров.

### Маршрут и параметр пути (route/path parameter)

Маршрут — это правило, которое связывает HTTP-метод и шаблон пути с обработчиком. В `"/items/:id".GET(http::get_item)` часть `:id` обозначает переменную часть пути. Для адреса `/items/550e8400-e29b-41d4-a716-446655440000` Ohkami передаст этот участок в `Path(id): Path<&str>`. Остальная часть адреса должна совпадать с шаблоном.

```rust
use ohkami::{Response, claw::Path};
use uuid::Uuid;

pub async fn get_item(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid item id");
    };
    println!("Requested item: {id}");
    Response::NotImplemented()
}
```

`Path` извлекает текст из адреса, а `Uuid::parse_str` проверяет формат идентификатора. При ошибочном ID возвращаем `400 Bad Request`. `501 Not Implemented` в примере означает: запрос разобран, но чтение товара ещё не подключено. 

Текущий `ItemsStorage::get` возвращает `Result<()>`, а не `Item`, поэтому на его основе пока нельзя отправить клиенту название и цену; для этого нужно изменить контракт и преобразовывать строку БД в сущность.

### Параметры строки запроса (query parameters)

Параметры после знака `?` служат, например, для настройки списка: `GET /items?limit=10`. Они не являются частью пути `/items/:id`: здесь `limit=10` задаёт ограничение количества результатов. В Ohkami `Query` извлекает их в структуру с `Deserialize`:

```rust
use ohkami::{Response, claw::Query};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListItemsQuery {
    limit: Option<usize>,
}

pub async fn list_items(Query(query): Query<ListItemsQuery>) -> Response {
    println!("Limit: {:?}", query.limit);
    Response::NotImplemented()
}

// Дополнительное правило в Ohkami::new: "/items".GET(http::list_items)
```

Поле `Option` позволяет вызывать `/items` без `limit`: тогда в поле будет `None`. Этот дополнительный маршрут и обработчик пока **не добавлены** в проект, а лишь показывают способ чтения query-параметра; фильтрации БД здесь ещё нет.

## `PUT` запросы

`PUT` в нашем примере предназначен для обновления товара с известным ID. Идентификатор возьмём из пути `/items/:id`, а новые `name` и `price` — из JSON-тела. Одновременно используем уже разобранные `Path` и `Json`:

### ID в пути и новые значения в теле

```rust
use ohkami::{Response, claw::{Json, Path}};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateItemPayload {
    name: String,
    price: f32,
}

pub async fn update_item(
    Path(id): Path<&str>,
    Json(payload): Json<UpdateItemPayload>,
) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid item id");
    };
    println!("Update {id}: {} / {}", payload.name, payload.price);
    Response::NotImplemented()
}
```

Пример запроса (после регистрации `"/items/:id".PUT(http::update_item)`):

```shell
curl -i -X PUT http://localhost:3000/items/550e8400-e29b-41d4-a716-446655440000 -H 'Content-Type: application/json' -d '{"name":"Новый карандаш","price":30}'
```

Чтобы действительно обновить товар, нужно получить его из БД, применить изменения и вызвать `ItemsStorage::update(&item)`. Пока `get` не возвращает `Item`, этот шаг нельзя корректно показать как завершённую операцию. Ответ `501` честно указывает на это; не нужно возвращать `200 OK` за одну только печать входных данных.

## `DELETE` запросы

`DELETE` удаляет товар по ID. Тело с названием и ценой для выбора товара не нужно: достаточно пути `/items/:id`.

### ID в пути

```rust
use ohkami::{Response, claw::Path};
use uuid::Uuid;

pub async fn remove_item(Path(id): Path<&str>) -> Response {
    let Ok(id) = Uuid::parse_str(id) else {
        return Response::BadRequest().with_text("invalid item id");
    };
    println!("Remove item: {id}");
    Response::NotImplemented()
}
```

```shell
curl -i -X DELETE http://localhost:3000/items/550e8400-e29b-41d4-a716-446655440000
```

В текущем контракте `ItemsStorage::remove` принимает целый `Item`, а не ID. Для настоящего удаления нужно сначала загрузить товар (после доработки `get`) либо изменить контракт удаления. Пока этого нет, `501` не создаёт ложного впечатления, что товар удалён.

## Ошибки при работе с БД

Ошибка разбора ID (`400`) и ошибка чтения или записи БД (`500`) — разные ситуации. Когда репозиторий возвращает `Err`, можно сопоставить варианты `Result` и вернуть `500 Internal Server Error`, а не сообщать об успехе. Например, **после получения пригодного репозитория**:

```rust
use crate::contracts::ItemsStorage;
use crate::entities::Item;
use ohkami::Response;

async fn save_item(storage: &impl ItemsStorage, item: Item) -> Response {
    match storage.add(item).await {
        Ok(()) => Response::Created().with_text("item added"),
        Err(error) => {
            eprintln!("Failed to add item: {error}");
            Response::InternalServerError().with_text("database error")
        }
    }
}
```

Это пример обработки результата записи, а не уже готовое подключение пула. Подробности ошибки выводятся на сервере, клиент получает общее сообщение. Отсутствие товара — отдельный случай (обычно `404 Not Found`), но нынешний `get` не отделяет его от других ошибок; для точного ответа надо сначала изменить контракт получения.

## Ответы сервера: `Response` и `Serialize`

До сих пор обработчики возвращали строки вроде `"added item"`. По такой строке клиент не узнает ID созданного товара и не сможет удобно разобрать ответ. Поэтому мы решили **отрефакторить ответы обработчиков**: возвращать подходящий HTTP-статус и JSON с именованными полями.

### Что содержит `Response`

HTTP-ответ состоит из статуса, заголовков и тела. Например, `Response::Created()` задаёт статус `201 Created`, а `.with_json(data)` помещает сериализованные данные в тело и задаёт тип содержимого `application/json`. 

Для чтения существующего товара подойдёт `Response::OK()` (`200`); при неверном ID — `BadRequest()` (`400`), при отсутствующем товаре — `NotFound()` (`404`), при сбое БД — `InternalServerError()` (`500`).

### `Serialize` для модели ответа

Ранее `#[derive(Deserialize)]` помогал превратить входящий JSON в `AddItemPayload`. Для обратного направления нужен `#[derive(Serialize)]`: он позволяет превратить Rust-структуру в JSON. Отдельная модель ответа показывает только те поля, которые мы хотим отправить клиенту, без внутренних полей самого `Item`:

```rust
use ohkami::Response;
use serde::Serialize;

use crate::entities::Item;

#[derive(Serialize)]
pub struct ItemResponse {
    id: String,
    name: String,
    price: f32,
}

fn created_item_response(item: &Item) -> Response {
    Response::Created().with_json(ItemResponse {
        id: item.id.to_string(),
        name: item.name.clone(),
        price: item.price,
    })
}
```

`ItemsStorage::add(item)` забирает `Item` во владение. Поэтому ответ можно подготовить с помощью `created_item_response(&item)` **до** вызова `add`, но отправлять его клиенту только при `Ok(())`. 

Ответ будет иметь статус `201 Created`, заголовок `Content-Type: application/json` и тело вида `{"id":"...","name":"Карандаш","price":25.5}`. Пример функции **не выполняет запись**: сначала нужно решить описанную выше проблему возврата `Client` в пул и вызвать репозиторий. 

Аналогично `GET` сможет вернуть `Response::OK().with_json(...)` только после того, как `ItemsStorage::get` начнёт возвращать найденный товар.

Ошибки тоже можно возвращать в едином формате, если клиенту нужен JSON, а не текст:

```rust
#[derive(Serialize)]
struct ErrorResponse {
    message: &'static str,
}

let response = Response::InternalServerError().with_json(ErrorResponse {
    message: "database error",
});
```

