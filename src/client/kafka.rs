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