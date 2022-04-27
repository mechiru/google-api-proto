#[cfg(any(feature = "google-iam-admin-v1"))]
pub mod admin;
#[cfg(any(feature = "google-iam-credentials-v1"))]
pub mod credentials;
#[cfg(any(feature = "google-iam-v1", feature = "google-iam-v1-logging"))]
pub mod v1;
#[cfg(any(feature = "google-iam-v1beta"))]
pub mod v1beta;
