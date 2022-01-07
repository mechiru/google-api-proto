#[cfg(any(
    feature = "google-ads-googleads-v7-common",
    feature = "google-ads-googleads-v7-enums",
    feature = "google-ads-googleads-v7-errors",
    feature = "google-ads-googleads-v7-resources",
    feature = "google-ads-googleads-v7-services",
))]
pub mod v7;

#[cfg(any(
    feature = "google-ads-googleads-v8-common",
    feature = "google-ads-googleads-v8-enums",
    feature = "google-ads-googleads-v8-errors",
    feature = "google-ads-googleads-v8-resources",
    feature = "google-ads-googleads-v8-services",
))]
pub mod v8;

#[cfg(any(
    feature = "google-ads-googleads-v9-common",
    feature = "google-ads-googleads-v9-enums",
    feature = "google-ads-googleads-v9-errors",
    feature = "google-ads-googleads-v9-resources",
    feature = "google-ads-googleads-v9-services",
))]
pub mod v9;