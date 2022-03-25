#[cfg(any(feature = "google-cloud-notebooks-logging-v1",))]
pub mod logging;

#[cfg(any(feature = "google-cloud-notebooks-v1",))]
pub mod v1;

#[cfg(any(feature = "google-cloud-notebooks-v1beta1",))]
pub mod v1beta1;
