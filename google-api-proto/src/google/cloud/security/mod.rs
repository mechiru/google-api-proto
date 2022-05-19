#[cfg(
    any(
        feature = "google-cloud-security-privateca-v1",
        feature = "google-cloud-security-privateca-v1beta1",
    )
)]
pub mod privateca;
#[cfg(any(feature = "google-cloud-security-publicca-v1beta1"))]
pub mod publicca;
