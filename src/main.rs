use std::fs;

use axum_demo::{error::AppError, route::create_route, setting::AppConfig};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let file_contents =
        fs::read_to_string("/Users/yiny/rust_projects/axum-demo/src/config.example.yaml")
            .expect("LogRocket: Should have been able to read the file");
    let config: AppConfig =
        serde_yaml::from_str(&file_contents).map_err(|_| panic!("read config failed"))?;

    let app = create_route(config).await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    Ok(axum::serve(listener, app)
        .await
        .map_err(|_| panic!("start server error"))?)
}
