#[cfg(any(
    feature = "google-cloud-bigquery-migration-tasks-assessment-v2alpha",
    feature = "google-cloud-bigquery-migration-tasks-translation-v2alpha",
))]
pub mod tasks;

#[cfg(any(feature = "google-cloud-bigquery-migration-v2alpha",))]
pub mod v2alpha;
