# google-api-proto

[![ci](https://github.com/mechiru/google-api-proto/workflows/ci/badge.svg)](https://github.com/mechiru/google-api-proto/actions?query=workflow:ci)
[![doc](https://github.com/mechiru/google-api-proto/workflows/doc/badge.svg)](https://mechiru.github.io/google-api-proto/google-api-proto/index.html)
[![version](https://img.shields.io/crates/v/google-api-proto.svg)](https://crates.io/crates/google-api-proto)

This library generated from [googleapis/googleapis][] using [tonic-build][].

## Overview
This library contains all the code generated from the [googleapis/googleapis][].

When using each product API, you must explicitly include it in your build using a feature flag.<br>
For example, if you want to use [Cloud Pub/Sub](https://cloud.google.com/pubsub), write `features = ["google-pubsub-v1"]` to Cargo.toml.

The feature name is the period of the package name of each proto file, replaced by a hyphen.
If you specify a package, it will automatically load the dependent packages and include them in the build.
It means that `features = ["google-spanner-admin-database-v1"]` is the same as the code below:
```rust
pub mod google {
    pub mod api {
        // generated code
    }
    pub mod iam {
        pub mod v1 {
            // generated code
        }
    }
    pub mod longrunning {
        // generated code
    }
    pub mod r#type {
        // generated code
    }
    pub mod rpc {
        // generated code
    }
    pub mod spanner {
        pub mod admin {
            pub mod database {
                pub mod v1 {
                    // generated code
                }
            }
        }
    }
}
```

In addition, multiple features can be specified.<br>
The list of available features can be found [here](./google-api-proto/Cargo.toml).

## Version matrices
| google-api-proto | tonic | tonic-build |
|------------------|-------|-------------|
| 1.0.0 ≦          | 0.6.x | 0.6.x       |

## Example
The complete code can be found [here](./examples/src/pubsub.rs).

Cargo.toml:
```toml
[dependencies]
# For runtime
tokio = { version = "1.15", features = ["macros", "rt-multi-thread"] }
# For google authentication
google-authz = { version = "1.0.0-alpha.1", features = ["tonic"] }
# For gRPC
tonic = { version = "0.6", features = ["tls", "tls-webpki-roots"] }
prost = { version = "0.9" }
prost-types = { version = "0.9" }
google-api-proto = { version = "*", features = ["google-spanner-admin-database-v1"] }
```

main.rs:
```rust
use std::env;

use google_api_proto::google::spanner::admin::database::v1::{
    database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
};
use google_authz::GoogleAuthz;
use tonic::{transport::Channel, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let project = "google project id";
    let instance = "spanner instance name";

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
```

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.

<!-- links -->
[googleapis/googleapis]: https://github.com/googleapis/googleapis
[tonic-build]: https://github.com/hyperium/tonic/tree/master/tonic-build
