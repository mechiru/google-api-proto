#[cfg(any(feature = "google-cloud-retail-logging"))]
pub mod logging;
#[cfg(any(feature = "google-cloud-retail-v2"))]
pub mod v2;
#[cfg(any(feature = "google-cloud-retail-v2alpha"))]
pub mod v2alpha;
#[cfg(any(feature = "google-cloud-retail-v2beta"))]
pub mod v2beta;
