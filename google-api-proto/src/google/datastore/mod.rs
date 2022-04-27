#[cfg(
    any(
        feature = "google-datastore-admin-v1",
        feature = "google-datastore-admin-v1beta1",
    )
)]
pub mod admin;
#[cfg(any(feature = "google-datastore-v1"))]
pub mod v1;
#[cfg(any(feature = "google-datastore-v1beta3"))]
pub mod v1beta3;
