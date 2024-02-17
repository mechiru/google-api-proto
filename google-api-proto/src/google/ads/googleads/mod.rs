#[cfg(
    any(
        feature = "google-ads-googleads-v14-common",
        feature = "google-ads-googleads-v14-enums",
        feature = "google-ads-googleads-v14-errors",
        feature = "google-ads-googleads-v14-resources",
        feature = "google-ads-googleads-v14-services",
    )
)]
pub mod v14;
#[cfg(
    any(
        feature = "google-ads-googleads-v15-common",
        feature = "google-ads-googleads-v15-enums",
        feature = "google-ads-googleads-v15-errors",
        feature = "google-ads-googleads-v15-resources",
        feature = "google-ads-googleads-v15-services",
    )
)]
pub mod v15;
