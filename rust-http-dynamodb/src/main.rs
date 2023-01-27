mod definition;
mod service;

use std::{str::FromStr, string::ParseError};

use definition::{Ordering, ReportCursor, ReportReason};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use service::{GetReportsInput, ReportService};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let qs = event.query_string_parameters();

    let filter = match qs.first("reason").map(ReportReason::from_str) {
        None => None,
        Some(Ok(filter)) => Some(filter),
        Some(Err(_)) => return make_err_response(400, "incorrect reason"),
    };

    let ordering = match qs.first("ordering").map(Ordering::from_str) {
        None => Ordering::default(),
        Some(Ok(ordering)) => ordering,
        Some(Err(_)) => return make_err_response(400, "incorrect ordering"),
    };

    let cursor = match qs.first("token").map(ReportCursor::from_str) {
        None => None,
        Some(Ok(cursor)) => Some(cursor),
        Some(Err(_)) => return make_err_response(400, "incorrect next page token"),
    };

    let service = ReportService::default();

    let input = GetReportsInput {
        filter,
        ordering,
        cursor,
    };

    let r = service.get_reports(input).await;
    tracing::info!(result = ?r);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;
    Ok(resp)
}

trait QueryMapExt {
    fn parse_optional_first<T: FromStr>(&self, key: &str) -> Result<T, ParseError>;
}

fn make_err_response<T: Into<Body>>(status_code: u16, body: T) -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(status_code)
        .header("content-type", "text/html")
        .body(body.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
