use crate::models::item::Item;
use std::future::Future;
use uuid::Uuid;

pub trait ItemStorage {
    fn add(
        &self,
        item: Item,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn get(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Item, Box<dyn std::error::Error>>> + Send;
    fn remove(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn update(
        &self,
        item: &Item,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
