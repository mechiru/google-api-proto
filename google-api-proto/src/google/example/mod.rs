#[cfg(any(feature = "google-example-endpointsapis-v1",))]
pub mod endpointsapis;

#[cfg(any(feature = "google-example-library-v1",))]
pub mod library;

#[cfg(any(
    feature = "google-example-showcase-v1",
    feature = "google-example-showcase-v1beta1",
    feature = "google-example-showcase-v1beta2",
    feature = "google-example-showcase-v1beta3",
))]
pub mod showcase;
