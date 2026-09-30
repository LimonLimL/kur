use crate::models::order::Order;
use std::future::Future;
use uuid::Uuid;

pub trait OrderStorage {
    fn add(
        &self,
        order: Order,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn get(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Order, Box<dyn std::error::Error>>> + Send;
    fn remove(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn update(
        &self,
        order: &Order,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
