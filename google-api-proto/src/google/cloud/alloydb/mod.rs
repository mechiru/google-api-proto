#[cfg(
    any(
        feature = "google-cloud-alloydb-connectors-v1",
        feature = "google-cloud-alloydb-connectors-v1alpha",
        feature = "google-cloud-alloydb-connectors-v1beta",
    )
)]
pub mod connectors;
#[cfg(any(feature = "google-cloud-alloydb-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-alloydb-v1alpha"))]
pub mod v1alpha;
#[cfg(any(feature = "google-cloud-alloydb-v1beta"))]
pub mod v1beta;
