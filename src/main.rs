// #[tokio::main]
// async fn main() {
//     // let file_contents =
//     //     fs::read_to_string("/Users/yiny/rust_projects/axum-demo/src/config.example.yaml").expect("LogRocket: Should have been able to read the file");
//     // println!("{file_contents}");
//     // let config: GlobalConfig = serde_yaml::from_str(&file_contents).unwrap();
//     // println!("{config:?}");
//     run().await
// }
mod error;
mod middleware;
mod model;
mod route;

use route::create_route;

#[tokio::main]
async fn main() {
    let app = create_route();
    // .expect("Producer creation error");
    // let futures = (0..5)
    //     .map(|i| async move {
    //         // The send operation on the topic returns a future, which will be
    //         // completed once the result or failure from Kafka is received.
    //         let delivery_status = producer
    //             .send(
    //                 FutureRecord::to("quickstart-events")
    //                     .payload(&format!("Message {}", i))
    //                     .key(&format!("Key {}", i))
    //                     .headers(OwnedHeaders::new().insert(Header {
    //                         key: "header_key",
    //                         value: Some("header_value"),
    //                     })),
    //                 Duration::from_secs(0),
    //             )
    //             .await;
    //         info!("Delivery status for message {} received", i);
    //         delivery_status
    //     })
    //     .collect::<Vec<_>>();

    // for future in futures {
    //     info!("Future completed. Result: {:?}", future.await);
    // }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
