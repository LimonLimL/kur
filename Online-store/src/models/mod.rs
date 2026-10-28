// Сущности магазинов, заказов и доставок используются косвенно: их конструкторы
// вызываются адаптерами БД, которые подключатся к HTTP-слою в следующих этапах.
#![allow(dead_code)]

pub mod delivery;
pub mod item;
pub mod online_store;
pub mod order;
pub mod value_objects;
