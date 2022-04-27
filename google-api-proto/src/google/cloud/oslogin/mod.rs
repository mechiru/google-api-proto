#[cfg(any(feature = "google-cloud-oslogin-common"))]
pub mod common;
#[cfg(any(feature = "google-cloud-oslogin-v1"))]
pub mod v1;
#[cfg(any(feature = "google-cloud-oslogin-v1alpha"))]
pub mod v1alpha;
#[cfg(any(feature = "google-cloud-oslogin-v1beta"))]
pub mod v1beta;
