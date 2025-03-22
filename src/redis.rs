use crate::app::RedisJob;
use anyhow::{anyhow, Result};
use redis::aio::MultiplexedConnection;
use redis_work_queue::{Item, WorkQueue};
use std::time::Duration;

pub struct RedisQueue {
    pub redis: MultiplexedConnection,
    pub work_queue: WorkQueue,
}

impl RedisQueue {
    pub async fn insert_job(&mut self, job: RedisJob) -> Result<()> {
        let item = Item::from_json_data(&job)?;
        match self.work_queue.add_item(&mut self.redis, &item).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(anyhow!("Job ID Already Exists in Queue")),
            Err(err) => Err(anyhow!("{}", err)),
        }
    }
    pub async fn get_job(&mut self) -> Result<Item> {
        match self
            .work_queue
            .lease(&mut self.redis, None, Duration::from_secs(5))
            .await?
        {
            Some(job) => Ok(job),
            None => Err(anyhow!("Failed to get job.")),
        }
    }
    pub async fn complete(&mut self, item: &Item) -> Result<bool> {
        self.work_queue
            .complete(&mut self.redis, item)
            .await
            .map_err(|err| anyhow!("{}", err))
    }
}
