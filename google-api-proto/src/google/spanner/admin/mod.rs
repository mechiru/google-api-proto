#[cfg(any(feature = "google-spanner-admin-database-v1"))]
pub mod database;
#[cfg(any(feature = "google-spanner-admin-instance-v1"))]
pub mod instance;
