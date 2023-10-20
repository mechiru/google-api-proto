#[cfg(
    any(
        feature = "google-cloud-redis-cluster-v1",
        feature = "google-cloud-redis-cluster-v1beta1",
    )
)]
pub mod cluster;
#[cfg(any(feature = "google-cloud-redis-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-redis-v1beta1"))]
pub mod v1beta1;
