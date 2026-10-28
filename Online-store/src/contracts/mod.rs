// Хранилища магазинов, заказов и доставок будут подключены к HTTP-обработчикам
// в следующих этапах, поэтому их неиспользование пока не считается ошибкой.
#![allow(dead_code)]

pub mod delivery_storage;
pub mod item_storage;
pub mod online_store_storage;
pub mod order_storage;
