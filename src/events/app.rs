use crate::notification::Notification;
use crate::updater::UpdaterClient;
use actix_web::dev::HttpServiceFactory;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::HttpResponse;
use actix_web::Result;
use serde_json::json;

async fn update_event<U: UpdaterClient + Clone + 'static>(
    updater: Data<U>,
    n: Json<Notification>,
) -> Result<HttpResponse> {
    updater.update(n.0);
    Ok(HttpResponse::Ok().json(json!({})))
}

pub fn update_app<U: UpdaterClient + Clone + Send + 'static>(
    updater: U,
) -> impl HttpServiceFactory {
    web::scope("/update")
        .data(updater)
        .service(web::resource("").route(web::post().to(update_event::<U>)))
}
