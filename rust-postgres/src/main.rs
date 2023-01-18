use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(sqlx::FromRow)]
struct Country {
    country: String,
    count: i64,
}

#[derive(Deserialize)]
struct Request {
    num: i64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .without_time()
        .with_line_number(true)
        .init();

    let pool = &PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DB_URL")?.as_str())
        .await?;

    run(service_fn(|event| function_handler(event, pool))).await
}

async fn function_handler(event: LambdaEvent<Request>, pool: &Pool<Postgres>) -> Result<(), Error> {
    let Request { num } = event.payload;

    sqlx::query_as::<_, Country>("SELECT * FROM country WHERE count > ?")
        .bind(num)
        .fetch_all(pool)
        .await?
        .iter()
        .for_each(|country| println!("country = {}, count = {}", country.country, country.count));

    Ok(())
}
