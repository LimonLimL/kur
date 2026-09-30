mod contracts;
mod contracts_impl;
mod models;

use crate::contracts::item_storage::ItemStorage;
use crate::contracts_impl::item_sqlite_storage::ItemSqliteStorage;
use crate::models::item::Item;
use smol;
use std::collections::HashMap;

fn main() {
    smol::block_on(async {
        println!("Запуск приложения...");

        // Путь должен совпадать с .env и реальной папкой
        let db_url = "sqlite://databases/data.db";
        let item_repo = ItemSqliteStorage::new(db_url).await;

        let new_item = Item::create_new(
            "Электрочайник Bosch".to_string(),
            "Bosch".to_string(),
            "TWK7601".to_string(),
            HashMap::from([
                ("color".to_string(), "black".to_string()),
                ("power".to_string(), "2400W".to_string()),
            ]),
            4500.0,
            24,
            "images/bosch_kettle.jpg".to_string(),
        );

        match item_repo.add(new_item).await {
            Ok(_) => println!("Товар успешно добавлен!"),
            Err(e) => eprintln!("Ошибка: {}", e),
        }
    });
}
