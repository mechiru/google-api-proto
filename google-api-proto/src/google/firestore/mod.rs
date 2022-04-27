#[cfg(
    any(
        feature = "google-firestore-admin-v1",
        feature = "google-firestore-admin-v1beta1",
        feature = "google-firestore-admin-v1beta2",
    )
)]
pub mod admin;
#[cfg(any(feature = "google-firestore-bundle"))]
pub mod bundle;
#[cfg(any(feature = "google-firestore-v1"))]
pub mod v1;
#[cfg(any(feature = "google-firestore-v1beta1"))]
pub mod v1beta1;
