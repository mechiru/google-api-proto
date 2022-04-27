#[cfg(any(feature = "grafeas-v1"))]
pub mod v1;
#[cfg(
    any(
        feature = "grafeas-v1beta1",
        feature = "grafeas-v1beta1-attestation",
        feature = "grafeas-v1beta1-build",
        feature = "grafeas-v1beta1-deployment",
        feature = "grafeas-v1beta1-discovery",
        feature = "grafeas-v1beta1-image",
        feature = "grafeas-v1beta1-package",
        feature = "grafeas-v1beta1-provenance",
        feature = "grafeas-v1beta1-source",
        feature = "grafeas-v1beta1-vulnerability",
    )
)]
pub mod v1beta1;
