#[cfg(
    any(
        feature = "google-analytics-admin-v1alpha",
        feature = "google-analytics-admin-v1beta",
    )
)]
pub mod admin;
#[cfg(
    any(
        feature = "google-analytics-data-v1alpha",
        feature = "google-analytics-data-v1beta",
    )
)]
pub mod data;
