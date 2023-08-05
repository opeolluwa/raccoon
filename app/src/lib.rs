use axum::{
    async_trait,
    extract::State,
    extract::{FromRequestParts, MatchedPath, Path},
    http::{request::Parts, HeaderMap, Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    RequestPartsExt, Router,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use std::{collections::HashMap, env, net::SocketAddr, time::Duration};
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// simple_logger::init().unwrap();
// simple_logger::SimpleLogger::new().env().init().unwrap();
pub async fn run() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_connection_string =
        env::var("DATABASE_URL").expect("database URL is not provided in env variable");
    println!("the database conn is {}", database_connection_string);

    let mut opt = ConnectOptions::new(database_connection_string);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema".to_owned()); // Setting default PostgreSQL schema

    let connection = Database::connect(opt)
        .await
        .expect("error connecting to database ");
    /*   Migrator::up(&connection, None)
    .await
    .expect("error running database migrations"); */
    //initialize cors layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let trace = TraceLayer::new_for_http();

    // build our application with some routes
    let app = Router::new()
        .route("/:version/foo", get(handler))
        .layer(trace)
        .layer(cors);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(version: Version) -> Html<&'static str> {
    let html = format!("<h1>received request with version {:?}<h1>", version);
    Html("<h1>Hello, World!</h1>")
}

#[derive(Debug)]
enum Version {
    V1,
    // V2,
    // V3,
}

#[async_trait]
impl<S> FromRequestParts<S> for Version
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(Version::V1),
            // "v2" => Ok(Version::V2),
            // "v3" => Ok(Version::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}
