#[cfg(
    any(
        feature = "google-cloud-aiplatform-v1-schema-predict-instance",
        feature = "google-cloud-aiplatform-v1-schema-predict-params",
        feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
    )
)]
pub mod predict;
#[cfg(any(feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition"))]
pub mod trainingjob;
