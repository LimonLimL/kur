mod contracts;
mod contracts_impl;
mod http;
mod models;

use std::sync::Arc;

use ohkami::fang::Context;
use ohkami::{Ohkami, Route};

use crate::contracts_impl::managed_connection_pool::ManagedConnectionPool;
use crate::http::RequestContext;

/// Путь к файлу базы данных относительно корня пакета.
const DATABASE_URL: &str = "database/data.db";

/// Максимальное количество одновременно открытых соединений с базой данных.
const MAX_CONNECTIONS_COUNT: usize = 10;

/// Адрес, на котором запускается HTTP сервер.
const SERVER_ADDRESS: &str = "localhost:3000";

fn main() {
    smol::block_on(async {
        // Один раз создаём управляемый пул соединений и регистрируем его
        // в контексте — обработчики будут получать пул для каждого запроса.
        let connection_pool =
            ManagedConnectionPool::initialize(DATABASE_URL.to_string(), MAX_CONNECTIONS_COUNT)
                .await;
        let request_context = RequestContext::new(Arc::new(connection_pool));

        println!("HTTP сервер запущен: http://{SERVER_ADDRESS}");

        Ohkami::new((
            Context::new(request_context),
            "/add-item".POST(http::add_item),
            "/items/:id"
                .GET(http::get_item)
                .PUT(http::update_item)
                .DELETE(http::remove_item),
        ))
        .howl(SERVER_ADDRESS)
        .await;
    })
}
