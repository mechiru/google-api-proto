#[cfg(any(feature = "google-cloud-discoveryengine-logging"))]
pub mod logging;
#[cfg(any(feature = "google-cloud-discoveryengine-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-discoveryengine-v1alpha"))]
pub mod v1alpha;
#[cfg(any(feature = "google-cloud-discoveryengine-v1beta"))]
pub mod v1beta;
