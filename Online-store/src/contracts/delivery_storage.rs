use crate::models::delivery::Delivery;
use std::future::Future;
use uuid::Uuid;

pub trait DeliveryStorage {
    fn add(
        &self,
        delivery: Delivery,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn get(
        &self,
        order_id: Uuid,
    ) -> impl Future<Output = Result<Delivery, Box<dyn std::error::Error>>> + Send;
    fn remove(
        &self,
        order_id: Uuid,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn update(
        &self,
        delivery: &Delivery,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
