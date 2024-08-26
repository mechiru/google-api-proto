use std::env;

use google_api_proto::google::pubsub::v1::{publisher_client::PublisherClient, ListTopicsRequest};
use google_authz::GoogleAuthz;
use tonic::{transport::Channel, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let project = env::args().nth(1).expect("cargo run --bin pubsub -- <GCP_PROJECT_ID>");

    let channel = Channel::from_static("https://pubsub.googleapis.com").connect().await?;
    let channel = GoogleAuthz::new(channel).await;

    let mut client = PublisherClient::new(channel);
    let response = client
        .list_topics(Request::new(ListTopicsRequest {
            project: format!("projects/{}", project),
            page_size: 10,
            ..Default::default()
        }))
        .await?;
    println!("response = {:#?}", response);

    Ok(())
}
