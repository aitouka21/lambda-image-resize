use image::GenericImageView;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use s3::{creds::Credentials, Bucket};
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    key: String,
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

    let bucket = &Bucket::new(
        &std::env::var("BUCKET").expect("missing BUCKET environment variable"),
        std::env::var("BUCKET_REGION")
            .expect("missing BUCKET_REGION environment variable")
            .parse()?,
        Credentials::default()?,
    )?;

    run(service_fn(|event| function_handler(event, bucket))).await
}

async fn function_handler(event: LambdaEvent<Request>, bucket: &Bucket) -> Result<(), Error> {
    let Request { key } = event.payload;

    match key.split(".").last() {
        Some("png") => {}
        _ => panic!("only accept png for testing"),
    }

    let res = bucket.get_object(&key).await?;
    tracing::info!(message = "get object done");
    let img = res.bytes();

    let img = resize(img);
    tracing::info!(message = "resize done");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis();

    bucket.put_object(format!("{}_{}", now, &key), &img).await?;
    tracing::info!(message = "put object done");

    Ok(())
}

fn resize(img: &[u8]) -> Vec<u8> {
    let img = image::load_from_memory_with_format(&img, image::ImageFormat::Png)
        .expect("failed to load image");

    let dimensions = img.dimensions();

    tracing::info!(original_dimensions = ?dimensions);

    let img = img.thumbnail(1000, 1000);

    let dimensions = img.dimensions();

    tracing::info!(new_dimensions = ?dimensions);

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png,
    )
    .unwrap();

    tracing::info!("resize done");
    bytes
}
