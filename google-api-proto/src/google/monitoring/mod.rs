#[cfg(any(feature = "google-monitoring-dashboard-v1"))]
pub mod dashboard;
#[cfg(any(feature = "google-monitoring-metricsscope-v1"))]
pub mod metricsscope;
#[cfg(any(feature = "google-monitoring-v3"))]
pub mod v3;
