#[cfg(
    any(
        feature = "google-cloud-recommender-logging-v1",
        feature = "google-cloud-recommender-logging-v1beta1",
    )
)]
pub mod logging;
#[cfg(any(feature = "google-cloud-recommender-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-recommender-v1beta1"))]
pub mod v1beta1;
