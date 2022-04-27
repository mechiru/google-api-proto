#[cfg(any(feature = "google-cloud-metastore-logging-v1"))]
pub mod logging;
#[cfg(any(feature = "google-cloud-metastore-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-metastore-v1alpha"))]
pub mod v1alpha;
#[cfg(any(feature = "google-cloud-metastore-v1beta"))]
pub mod v1beta;
