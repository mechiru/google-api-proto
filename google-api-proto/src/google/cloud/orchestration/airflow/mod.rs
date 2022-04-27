#[cfg(
    any(
        feature = "google-cloud-orchestration-airflow-service-v1",
        feature = "google-cloud-orchestration-airflow-service-v1beta1",
    )
)]
pub mod service;
