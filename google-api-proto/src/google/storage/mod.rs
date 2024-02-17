#[cfg(any(feature = "google-storage-control-v2"))]
pub mod control;
#[cfg(any(feature = "google-storage-v1"))]
pub mod v1;
#[cfg(any(feature = "google-storage-v2"))]
pub mod v2;
