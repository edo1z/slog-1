use thiserror::Error;
use tracing::{error, instrument, Span};

fn log_error_and_convert<E, T>(error: E) -> T
where
    E: Into<T>,
    T: std::fmt::Display + std::fmt::Debug,
{
    let err: T = error.into();
    error!("{} ({}:{})", err, file!(), line!());
    err
}

#[derive(Debug, Error)]
enum ServiceError {
    #[error("HogeError: {0}")]
    HogeError(String),
}

impl From<String> for ServiceError {
    fn from(err: String) -> Self {
        Self::HogeError(err)
    }
}

#[instrument(name="main/service", skip_all, fields(x = %x, y))]
async fn service(x: i32, y: i32) -> Result<(), ServiceError> {
    if x == y {
        Span::current().record("y", &y);
        log_error_and_convert::<String, ServiceError>("x == y".to_string());
    }
    Ok(())
}

#[instrument(skip_all, name="main/use_case", fields(x=%x))]
async fn use_case(x: i32, y: i32) {
    match service(x, y).await {
        Ok(_) => println!("use_case success"),
        Err(_) => println!("use_case fail"),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let x = 100;
    let y = 100;
    use_case(x, y).await;
}
