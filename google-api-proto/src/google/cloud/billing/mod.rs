#[cfg(
    any(
        feature = "google-cloud-billing-budgets-v1",
        feature = "google-cloud-billing-budgets-v1beta1",
    )
)]
pub mod budgets;
#[cfg(any(feature = "google-cloud-billing-v1"))]
pub mod v1;
