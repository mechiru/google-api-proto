#[cfg(any(feature = "google-appengine-legacy"))]
pub mod legacy;
#[cfg(any(feature = "google-appengine-logging-v1"))]
pub mod logging;
#[cfg(any(feature = "google-appengine-v1"))]
pub mod v1;
#[cfg(any(feature = "google-appengine-v1beta"))]
pub mod v1beta;
