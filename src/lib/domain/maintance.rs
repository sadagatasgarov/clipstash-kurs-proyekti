use crate::data::DatabasePool;
use crate::service;
use std::time::Duration;
use tokio::runtime::Handle;


pub struct Maintance;


impl Maintance {
    pub fn spawn(pool: DatabasePool, handle: Handle) -> Self {
        handle.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Err(e) = service::action::delete_expires(&pool).await {
                    eprintln!("failed to delete expired clips: {}", e)
                }

            }
        });
        Self
    }
}