use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use opentelemetry::global;
use opentelemetry_sdk::trace as sdk;

// use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::trace::Tracer; //Key, KeyValue
use opentelemetry_otlp::WithExportConfig;

use opentelemetry_sdk::Resource;

#[derive(Deserialize, Serialize)]
pub struct PredictionRequest {
    model_name: String,
    model_version: String,
}

async fn cache_predict(
    web::Json(p_req): web::Json<PredictionRequest>,
    redis: web::Data<redis::Client>,
) -> actix_web::Result<impl Responder> {
    let mut conn = redis
        .get_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;
    let uuid = Uuid::new_v4();
    let res = redis::cmd("SET")
        .arg(&[uuid.to_string(), serde_json::to_string(&p_req).unwrap()])
        .query_async::<_, String>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // not strictly necessary, but successful SET operations return "OK"
    if res == "OK" {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

async fn index() -> impl Responder {
    let tracer = global::tracer("request");
    tracer.in_span("index", |_cx| HttpResponse::Ok().body("Hello, world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // env_logger::init();

    let _tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(sdk::config().with_resource(Resource::new(vec![
            opentelemetry::KeyValue::new("service.name", "trace-http-demo"),
        ])))
        .install_simple()
        .expect("Error installing pipeline");
    let redis = redis::Client::open("redis://127.0.0.1:6379").unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(redis.clone()))
            .service(
                web::resource("/predict").route(web::post().to(cache_predict)), //.route(web::delete().to(del_stuff)),
            )
            //.wrap_fn(move |req, srv| {
            //    tracer.in_span("middleware", move |cx| {
            //        cx.span()
            //            .set_attribute(Key::new("path").string(req.path().to_string()));
            //        srv.call(req).with_context(cx)
            //    })
            //})
            .route("/", web::get().to(index))
        //.service(healthcheck)
        //.service(metrics)
        //.wrap(telemetry.metrics())
    })
    .workers(2)
    .bind(("127.0.0.1", 8487))?
    .run()
    .await?;
    Ok(())
}
