use std::env;

use google_api_proto::google::spanner::admin::database::v1::{
    database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
};
use google_authz::GoogleAuthz;
use tonic::{transport::Channel, Request};

const HELP: &str = "cargo run --bin spanner -- <GCP_PROJECT_ID> <SPANNER_INSTANCE>";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut args = env::args().skip(1);
    let project = args.next().expect(HELP);
    let instance = args.next().expect(HELP);

    let channel = Channel::from_static("https://spanner.googleapis.com").connect().await?;
    let channel = GoogleAuthz::new(channel).await;

    let mut client = DatabaseAdminClient::new(channel);
    let response = client
        .list_databases(Request::new(ListDatabasesRequest {
            parent: format!("projects/{}/instances/{}", project, instance),
            page_size: 100,
            ..Default::default()
        }))
        .await?;
    println!("response = {:#?}", response);

    Ok(())
}
