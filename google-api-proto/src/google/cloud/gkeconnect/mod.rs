#[cfg(
    any(
        feature = "google-cloud-gkeconnect-gateway-v1",
        feature = "google-cloud-gkeconnect-gateway-v1alpha1",
        feature = "google-cloud-gkeconnect-gateway-v1beta1",
    )
)]
pub mod gateway;
