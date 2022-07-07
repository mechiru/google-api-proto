#[cfg(any(feature = "google-cloud-functions-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-functions-v2"))]
pub mod v2;
#[cfg(any(feature = "google-cloud-functions-v2alpha"))]
pub mod v2alpha;
#[cfg(any(feature = "google-cloud-functions-v2beta"))]
pub mod v2beta;
