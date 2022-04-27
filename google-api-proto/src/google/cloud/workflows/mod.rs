#[cfg(
    any(
        feature = "google-cloud-workflows-executions-v1",
        feature = "google-cloud-workflows-executions-v1beta",
    )
)]
pub mod executions;
#[cfg(any(feature = "google-cloud-workflows-type"))]
pub mod r#type;
#[cfg(any(feature = "google-cloud-workflows-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-workflows-v1beta"))]
pub mod v1beta;
