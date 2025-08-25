use crate::{
    api::v1::auth::models::UserInfo,
    app::ApiError,
    db::event::{Event, EventData},
};
use actix_session::Session;
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, Scope,
};
use log::error;
use serde::Deserialize;

use crate::app::AppState;
use crate::auth::SessionAuth;

use utoipa::OpenApi;

use crate::db::user::UserData;

mod car;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/{event_id}/car", api = car::ApiDoc),
    ),
    paths(
        create_event,
        get_event,
        get_all_events,
        update_event,
        delete_event
    ),
    components(schemas(Event, EventData, UserData))
)]
pub(super) struct ApiDoc;

#[utoipa::path(
    responses(
        (status = 200, description = "Create New Event. Returns ID", body = i32),
        (status = 400, body = ApiError),
        (status = 401, body = ApiError),
        (status = 500, body = ApiError),
    )
)]
#[post("/", wrap = "SessionAuth")]
async fn create_event(
    data: web::Data<AppState>,
    session: Session,
    event: web::Json<EventData>,
) -> impl Responder {
    let user_id = match session.get::<UserInfo>("userinfo").ok().flatten() {
        Some(user) => user.id,
        None => {
            return HttpResponse::Unauthorized().json(ApiError::from(
                "Failed to get user data from session".to_string(),
            ))
        }
    };

    if let Err(errs) = event.validate() {
        return HttpResponse::BadRequest().json(ApiError::from(errs));
    }
    let result = Event::insert_new(&event, user_id, &data.db).await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError()
                .json(ApiError::from("Failed to create event".to_string()))
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get event specified by ID", body = Event),
        (status = 404, body = ApiError),
        (status = 500, body = ApiError),
    )
)]
#[get("/{event_id}", wrap = "SessionAuth")]
async fn get_event(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let event_id = path.into_inner();
    let result = Event::select_one(event_id, &data.db).await;

    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ApiError::from("Event not found".to_string())),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError()
                .json(ApiError::from("Failed to get event".to_string()))
        }
    }
}

#[derive(Deserialize)]
struct EventQueryParams {
    past: Option<bool>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get all events", body = [Event]),
        (status = 500, body = ApiError),
    )
)]
#[get("/", wrap = "SessionAuth")]
async fn get_all_events(
    data: web::Data<AppState>,
    params: web::Query<EventQueryParams>,
) -> impl Responder {
    let past: bool = params.past.unwrap_or(false);

    let result = crate::db::event::Event::select_all(past, &data.db).await;

    match result {
        Ok(events) => {
            if past {
                let reversed : Vec<&Event> = events.iter().rev().collect();

                return HttpResponse::Ok().json(reversed);
            }

            HttpResponse::Ok().json(events)
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError()
                .json(ApiError::from("Failed to get events".to_string()))
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Update event information"),
        (status = 400, body = ApiError),
        (status = 403, body = ApiError),
        (status = 404, body = ApiError),
        (status = 500, body = ApiError),

    )
)]
#[put("/{event_id}", wrap = "SessionAuth")]
async fn update_event(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
    event: web::Json<EventData>,
) -> impl Responder {
    let event_id = path.into_inner();
    let user_id = match session.get::<UserInfo>("userinfo").ok().flatten() {
        Some(user) => user.id,
        None => {
            return HttpResponse::Unauthorized().json(ApiError::from(
                "Failed to get user data from session".to_string(),
            ))
        }
    };

    if let Err(errs) = event.validate() {
        return HttpResponse::BadRequest().json(ApiError::from(errs));
    }

    let updated = Event::update(event_id, user_id, &event, &data.db).await;

    match updated {
        Ok(Some(_)) => HttpResponse::Ok().body("Event updated successfully"),
        Ok(None) => HttpResponse::NotFound().json(ApiError::from(
            "Event not found or you are not the creator".to_string(),
        )),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError()
                .json(ApiError::from("Failed to update event".to_string()))
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Delete Event"),
        (status = 401, body = ApiError),
        (status = 404, body = ApiError),
        (status = 500, body = ApiError),
    )
)]
#[delete("/{event_id}", wrap = "SessionAuth")]
async fn delete_event(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id = path.into_inner();
    let user_id = match session.get::<UserInfo>("userinfo").ok().flatten() {
        Some(user) => user.id,
        None => {
            return HttpResponse::Unauthorized().json(ApiError::from(
                "Failed to get user data from session".to_string(),
            ))
        }
    };

    let deleted = Event::delete(event_id, user_id, &data.db).await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Event deleted"),
        Ok(None) => HttpResponse::NotFound().json(ApiError::from(
            "Event not found or you are not the creator".to_string(),
        )),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError()
                .json(ApiError::from("Failed to delete event".to_string()))
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Added to Need Ride"),
        (status = 401, body = ApiError),
        (status = 404, body = ApiError),
        (status = 500, body = ApiError),
    )
)]
#[post("/{event_id}/needride", wrap = "SessionAuth")]
async fn need_ride(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id = path.into_inner();
    let user_id = match session.get::<UserInfo>("userinfo").ok().flatten() {
        Some(user) => user.id,
        None => {
            return HttpResponse::Unauthorized().json(ApiError::from(
                "Failed to get user data from session".to_string(),
            ))
        }
    };

    match crate::db::car::Car::user_in_car(event_id, &user_id, &data.db).await {
        Ok(true) => {
            return HttpResponse::BadRequest()
                .json(ApiError::from("User is already in a car.".to_string()))
        }
        Ok(false) => {}
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError().json(ApiError::from(
                "Failed to check user's occupancy in other cars".to_string(),
            ));
        }
    }

    match crate::db::needs_ride::NeedsRide::insert_new(user_id, event_id, &data.db).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().json(ApiError::from(
                "Failed to add user to need ride list".to_string(),
            ))
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Removed from Need Ride"),
        (status = 401, body = ApiError),
        (status = 404, body = ApiError),
        (status = 500, body = ApiError),
    )
)]
#[delete("/{event_id}/needride", wrap = "SessionAuth")]
async fn remove_need_ride(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id = path.into_inner();
    let user_id = match session.get::<UserInfo>("userinfo").ok().flatten() {
        Some(user) => user.id,
        None => {
            return HttpResponse::Unauthorized().json(ApiError::from(
                "Failed to get user data from session".to_string(),
            ));
        }
    };

    match crate::db::needs_ride::NeedsRide::delete(user_id, event_id, &data.db).await {
        Ok(_) => HttpResponse::Ok().body("Removed from need ride list"),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().json(ApiError::from(
                "Failed to remove user from need ride list".to_string(),
            ))
        }
    }
}

pub fn scope() -> Scope {
    web::scope("/event")
        .service(create_event)
        .service(get_event)
        .service(get_all_events)
        .service(update_event)
        .service(delete_event)
        .service(need_ride)
        .service(remove_need_ride)
        .service(car::scope())
}
