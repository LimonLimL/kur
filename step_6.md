# HTTP модуль для коммуникации с приложением.

Наш `main()` - точка входа в приложение, где инициализируются все необходимые компоненты и запускается HTTP сервер. Пока что он выглядит так:

```rust
mod contracts;
mod contracts_impl;
mod entities;

fn main() {
    println!("Hello, world!");
}
```

В нём ничего нет. Это нужно исправить и реализовать HTTP сервер, который будет обрабатывать входящие запросы.

## Ohkami. Библиотека для создания HTTP серверов на Rust.

Ohkami предоставляет удобные готовые компоненты, которые можно использовать для быстрого создания HTTP серверов и обработки входящих запросов.

Подключить библиотеку необходимо в `Cargo.toml`:

```toml
[dependencies.ohkami]
version = "0.24.9"
features = ["rt_smol"]
```

## Объявление HTTP обработчиков:

Для того, чтобы разрабатываемому сервису можно было сказать "сделать что-то", для этого необходимо иметь рычаги, за которые HTTP сервер можно "дёрнуть" и что-то вызвать.
Этими HTTP рычагами являются HTTP обработчики (http handlers). Они представляют собой функции, которые вызываются при получении HTTP запросов на определённые маршруты (routes).

Допустим, что нужно управлять складом товаров (по предметной области из примера).

Необходимо:

- Добавлять новые товары на склад.
- Изменить информацию о существующих товарах на складе.
- Удалять товары со склада.
- Получать информацию о товарах на складе.

Все эти действия будут реализованы через HTTP обработчики, которые будут привязаны к соответствующим маршрутам нашего сервера.

Нам нужно создать папку `http`, где будут храниться все HTTP обработчики нашего сервера.

Пример структуры проекта:

```
src/
├── http/
│   ├── mod.rs
├── contracts.rs
├── contracts_impl.rs
├── entities.rs
└── main.rs
```

Перейдем в `mod.rs`, внутри папки `http` и объявим необходимые HTTP обработчики:

```rust
use ohkami::Response;

pub async fn add_item() -> Response {
	todo!()
}

pub async fn remove_item() -> Response {
	todo!()
}

pub async fn update_item() -> Response {
	todo!()
}

pub async fn get_item() -> Response {
	todo!()
}
```

Реализуем внутри обработчиков простую логику, чтобы понять, что сервер вообще работает.

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

Эти обработчики пока ничего не делают, они просто будут возвращать какой-то текст. В дальнейшем мы превратим их в полноценные точки взаимодействия с нашим складом товаров.

## Создание и запуск HTTP сервера.

Перейдем в `main.rs` и создадим HTTP сервер, используя `ohkami`:

```rust
use ohkami::{Ohkami, Route};

mod contracts;
mod contracts_impl;
mod entities;
mod http;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        Ohkami::new((
            "/add-item".POST(http::add_item),
            "/remove-item".DELETE(http::remove_item),
            "/update-item".PUT(http::update_item),
            "/get-item".GET(http::get_item),
        ))
        .howl("localhost:3000")
        .await;
        Ok(())
    })
}
```

Объяснение:

```rust
smol::block_on(...)
```

`main` — обычная синхронная функция, поэтому внутри неё нельзя напрямую использовать `.await`. Выражение `async { ... }` создаёт асинхронную задачу (future), но само по себе не начинает её выполнение. `smol::block_on` выполняет эту задачу и ждёт её завершения, блокируя **текущий поток** `main`, а не запуская блок в отдельном потоке.

Внутри `smol::block_on(async { ... })` используется `Ohkami::new` для создания сервера с правилами маршрутизации.

Теперь разберём, как запрос попадает в обработчик. `Ohkami::new(( ... ))` получает список правил маршрутизации в виде кортежа: элементы внутри двойных скобок разделены запятыми. Каждое правило связывает **путь**, **HTTP-метод** и **функцию-обработчик**. Например:

```rust
"/add-item".POST(http::add_item)
```

`"/add-item"` — путь из адреса запроса, `POST` — HTTP-метод, а `http::add_item` — функция из модуля `http`. Запись `http::add_item` передаёт функцию серверу, **не вызывая её сразу**: здесь нет скобок `()`. Метод `.POST(...)` доступен благодаря `use ohkami::Route;` и создаёт правило «при запросе `POST /add-item` вызвать `http::add_item`». 

Аналогично другие строки связывают `DELETE /remove-item` с `http::remove_item`, `PUT /update-item` с `http::update_item` и `GET /get-item` с `http::get_item`.

Пока `Ohkami::new(...)` только собирает сервер с этими правилами: он ещё не принимает запросы. Вызов `.howl("localhost:3000").await` запускает сервер на адресе `localhost` и порту `3000`. Когда приходит запрос, Ohkami сравнивает его метод и путь с правилами, вызывает подходящий обработчик и отправляет клиенту возвращённый им `Response`. 

Например, запрос `POST http://localhost:3000/add-item` вызовет `add_item`, и клиент получит текст `added item`. Запрос `GET /add-item` не соответствует этому правилу, потому что метод другой.

Вызов `howl(...).await` ожидает, пока сервер работает. Поэтому до `Ok(())` выполнение обычно доходит только после завершения работы сервера. Возвращаемый тип `smol::io::Result<()>` означает, что `main` возвращает результат, а `Ok(())` обозначает успешное завершение.

Когда асинхронная операция ждёт данные из сети, `.await` позволяет уступить выполнение другим готовым задачам. Благодаря этому сервер может обрабатывать несколько запросов конкурентно.

## Проверка работы сервера.

Для начала сервер необходимо запустить. Сделать мы можем это командой:

```shell
cargo run
```

Для проверки работы сервера можно использовать команды в терминале `curl`.

Синтаксис следующий:

```shell
curl -i -X <HTTP_МЕТОД> <URL>
```

Например:

```shell
curl -i -X POST http://localhost:3000/add-item
curl -i -X GET http://localhost:3000/get-item
curl -i -X PUT http://localhost:3000/update-item
curl -i -X DELETE http://localhost:3000/remove-item
```

Это работает, потому что, был настроен роутер с соответствующими маршрутами для каждого HTTP-метода и пути:

```rust
Ohkami::new((
    "/add-item".POST(http::add_item),
    "/remove-item".DELETE(http::remove_item),
    "/update-item".PUT(http::update_item),
    "/get-item".GET(http::get_item),
))
```

А сами функции внутри возвращают объект `Response`, который библиотека транслирует в HTTP-ответ и отправляет клиенту.

```rust
pub async fn update_item() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_text("updated item");
    return response;
}
```