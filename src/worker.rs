use std::{collections::HashSet, env};

use anyhow::{anyhow, Result};
use log::error;
use redis_work_queue::{Item, KeyPrefix, WorkQueue};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{
    app::{RedisJob, SimpleRiderChange},
    db::{
        car::Car,
        event::Event,
        user::{UserData, UserRealm},
    },
    pings::PingClient,
    redis::RedisQueue,
};

struct RedisError {
    pub msg: String,
    pub should_retry: bool,
}

pub async fn main() -> Result<()> {
    let db = redis::Client::open(env::var("REDIS_URL").expect("REDIS_URL must be set"))?
        .get_multiplexed_async_connection()
        .await?;

    let work_queue = WorkQueue::new(KeyPrefix::from("rideboard"));

    let queue = RedisQueue {
        redis: db.clone(),
        work_queue,
    };

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await?;

    let pings = PingClient::new(
        env::var("PINGS_TOKEN").expect("PINGS_TOKEN must be set"),
        env::var("PINGS_JOIN_ROUTE").expect("PINGS_JOIN_ROUTE must be set"),
        env::var("PINGS_LEAVE_ROUTE").expect("PINGS_LEAVE_ROUTE must be set"),
        env::var("PINGS_ADD_ROUTE").expect("PINGS_ADD_ROUTE must be set"),
        env::var("PINGS_REMOVE_ROUTE").expect("PINGS_REMOVE_ROUTE must be set"),
    )?;

    work_loop(queue, db_pool, pings).await?;
    Ok(())
}

async fn get_simple_data(
    data: SimpleRiderChange,
    db_pool: &Pool<Postgres>,
) -> Result<(Event, Car, UserData)> {
    let rider = UserData::select_one(data.target_id, db_pool)
        .await?
        .ok_or(anyhow!("Rider does not exist"))?;
    Ok((
        Event::select_one(data.event_id, db_pool)
            .await?
            .ok_or(anyhow!("Event does not exist"))?,
        Car::select_one(data.event_id, data.car_id, db_pool)
            .await?
            .ok_or(anyhow!("Car does not exist"))?,
        rider,
    ))
}

async fn work(
    job: &Item,
    db_pool: &Pool<Postgres>,
    pings: &PingClient,
    queue: &mut RedisQueue,
) -> Result<(), RedisError> {
    let job_data: RedisJob = job.data_json().map_err(|_err| RedisError {
        msg: "Failed to Parse into Job".to_string(),
        should_retry: false,
    })?;
    match job_data {
        RedisJob::Join(data) => {
            let (event, car, rider) =
                get_simple_data(data, db_pool)
                    .await
                    .map_err(|err| RedisError {
                        msg: err.to_string(),
                        should_retry: false,
                    })?;
            if car.driver.realm != UserRealm::Csh {
                return Ok(());
            }
            pings
                .send_join(
                    car.driver.email.trim_end_matches("@csh.rit.edu"),
                    &rider.name,
                    &event.name,
                )
                .await
                .map_err(|err| RedisError {
                    msg: err.to_string(),
                    should_retry: true,
                })?
        }
        RedisJob::Leave(data) => {
            let (event, car, rider) =
                get_simple_data(data, db_pool)
                    .await
                    .map_err(|err| RedisError {
                        msg: err.to_string(),
                        should_retry: false,
                    })?;
            if car.driver.realm != UserRealm::Csh {
                return Ok(());
            }
            pings
                .send_leave(
                    car.driver.email.trim_end_matches("@csh.rit.edu"),
                    &rider.name,
                    &event.name,
                )
                .await
                .map_err(|err| RedisError {
                    msg: err.to_string(),
                    should_retry: true,
                })?;
            for rider in &event.needs_ride {
                queue
                    .insert_job(RedisJob::Opening(SimpleRiderChange {
                        event_id: event.id,
                        car_id: car.id,
                        target_id: rider.id.clone(),
                    }))
                    .await
                    .map_err(|err| RedisError {
                        msg: err.to_string(),
                        should_retry: false,
                    })?;
            }
        }
        RedisJob::RiderUpdate(data) => {
            let event = Event::select_one(data.event_id, db_pool)
                .await
                .map_err(|err| RedisError {
                    msg: err.to_string(),
                    should_retry: true,
                })?
                .ok_or(RedisError {
                    msg: "Event was missing from db.".to_string(),
                    should_retry: false,
                })?;
            let car = Car::select_one(data.event_id, data.car_id, db_pool)
                .await
                .map_err(|err| RedisError {
                    msg: err.to_string(),
                    should_retry: true,
                })?
                .ok_or(RedisError {
                    msg: "Car was missing from db.".to_string(),
                    should_retry: false,
                })?;
            let old_set: HashSet<String> = HashSet::from_iter(data.old_riders);
            let new_set: HashSet<String> = HashSet::from_iter(data.new_riders);
            let user_map = UserData::select_map(
                old_set
                    .difference(&new_set)
                    .chain(new_set.difference(&old_set))
                    .map(|s| s.to_string())
                    .collect(),
                db_pool,
            )
            .await
            .map_err(|err| RedisError {
                msg: err.to_string(),
                should_retry: true,
            })?;
            for removed in old_set.difference(&new_set) {
                let user = user_map.get(removed).ok_or(RedisError {
                    msg: "User was missing from map.".to_string(),
                    should_retry: false,
                })?;
                if user.realm != UserRealm::Csh {
                    continue;
                }
                pings
                    .send_remove(
                        user.email.trim_end_matches("@csh.rit.edu"),
                        &car.driver.name,
                        &event.name,
                    )
                    .await
                    .map_err(|err| RedisError {
                        msg: format!("Failed to send message: {}", err),
                        should_retry: true,
                    })?;
            }
            for added in new_set.difference(&old_set) {
                let user = user_map.get(added).ok_or(RedisError {
                    msg: "User was missing from map.".to_string(),
                    should_retry: false,
                })?;
                if user.realm != UserRealm::Csh {
                    continue;
                }
                pings
                    .send_add(
                        user.email.trim_end_matches("@csh.rit.edu"),
                        &car.driver.name,
                        &event.name,
                    )
                    .await
                    .map_err(|err| RedisError {
                        msg: format!("Failed to send message: {}", err),
                        should_retry: true,
                    })?;
            }
            let current_capacity = car.riders.map(|riders| riders.len()).unwrap_or(0) as i32;
            if current_capacity < car.max_capacity {
                for rider in &event.needs_ride {
                    queue
                        .insert_job(RedisJob::Opening(SimpleRiderChange {
                            event_id: event.id,
                            car_id: car.id,
                            target_id: rider.id.clone(),
                        }))
                        .await
                        .map_err(|err| RedisError {
                            msg: err.to_string(),
                            should_retry: false,
                        })?;
                }
            }
        }
        RedisJob::Opening(data) => {
            let (event, car, needs_ride) =
                get_simple_data(data, db_pool)
                    .await
                    .map_err(|err| RedisError {
                        msg: err.to_string(),
                        should_retry: false,
                    })?;
            if needs_ride.realm != UserRealm::Csh {
                return Ok(());
            }
            pings
                .send_opening(
                    needs_ride.email.trim_end_matches("@csh.rit.edu"),
                    &car.driver.name,
                    &event.name,
                )
                .await
                .map_err(|err| RedisError {
                    msg: err.to_string(),
                    should_retry: true,
                })?;
        }
    }
    Ok(())
}

pub async fn work_loop(
    mut queue: RedisQueue,
    db_pool: Pool<Postgres>,
    pings: PingClient,
) -> Result<()> {
    loop {
        // Wait for a job with no timeout and a lease time of 5 seconds.
        let job: Item = match queue.get_job().await {
            Ok(job) => job,
            Err(err) => {
                error!("{}", err);
                continue;
            }
        };
        match work(&job, &db_pool, &pings, &mut queue).await {
            // Mark successful jobs as complete
            Ok(()) => {
                queue.complete(&job).await?;
            }
            // Drop a job that should be retried - it will be returned to the work queue after
            // the (5 second) lease expires.
            Err(err) if err.should_retry => error!("{}", err.msg),
            // Errors that shouldn't cause a retry should mark the job as complete so it isn't
            // tried again.
            Err(err) => {
                error!("{}", err.msg);
                queue.complete(&job).await?;
            }
        }
    }
}
