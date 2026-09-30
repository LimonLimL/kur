use crate::models::online_store::OnlineStore;
use std::future::Future;
use uuid::Uuid;

pub trait OnlineStoreStorage {
    fn add(
        &self,
        store: OnlineStore,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn get(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<OnlineStore, Box<dyn std::error::Error>>> + Send;
    fn remove(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn update(
        &self,
        store: &OnlineStore,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
}
