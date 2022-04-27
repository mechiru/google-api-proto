#[cfg(any(feature = "google-cloud-aiplatform-logging"))]
pub mod logging;
#[cfg(
    any(
        feature = "google-cloud-aiplatform-v1",
        feature = "google-cloud-aiplatform-v1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
        feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
    )
)]
pub mod v1;
#[cfg(
    any(
        feature = "google-cloud-aiplatform-v1beta1",
        feature = "google-cloud-aiplatform-v1beta1-schema",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
        feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
    )
)]
pub mod v1beta1;
