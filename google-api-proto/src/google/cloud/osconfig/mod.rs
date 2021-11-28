#[cfg(any(
    feature = "google-cloud-osconfig-agentendpoint-v1",
    feature = "google-cloud-osconfig-agentendpoint-v1beta",
))]
pub mod agentendpoint;

#[cfg(any(feature = "google-cloud-osconfig-v1",))]
pub mod v1;

#[cfg(any(feature = "google-cloud-osconfig-v1alpha",))]
pub mod v1alpha;

#[cfg(any(feature = "google-cloud-osconfig-v1beta",))]
pub mod v1beta;
