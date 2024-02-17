#[cfg(
    any(
        feature = "google-spanner-admin-database-v1",
        feature = "google-spanner-admin-instance-v1",
    )
)]
pub mod admin;
#[cfg(any(feature = "google-spanner-executor-v1"))]
pub mod executor;
#[cfg(any(feature = "google-spanner-v1"))]
pub mod v1;
