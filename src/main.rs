use std::error::Error;
use tracing::{error, info, instrument};

#[instrument]
async fn process_data(data: &String) -> Result<(), Box<dyn Error>> {
    info!("処理中のデータ: {}", data);
    Ok(())
}

#[instrument]
async fn process_error(data: &String) -> Result<(), Box<dyn Error>> {
    info!("処理中のデータ: {}", data);
    Err("エラー!".into())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let data = "サンプルデータ".to_string();

    let _ = process_data(&data).await;

    if let Err(e) = process_error(&data).await {
        error!("エラー: {}", e);
    }

    info!("hoge");
}
