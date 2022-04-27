#[cfg(any(feature = "google-cloud-bigquery-storage-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-bigquery-storage-v1beta1"))]
pub mod v1beta1;
#[cfg(any(feature = "google-cloud-bigquery-storage-v1beta2"))]
pub mod v1beta2;
