#[cfg(any(feature = "google-container-v1"))]
pub mod v1;
#[cfg(any(feature = "google-container-v1alpha1"))]
pub mod v1alpha1;
#[cfg(any(feature = "google-container-v1beta1"))]
pub mod v1beta1;
