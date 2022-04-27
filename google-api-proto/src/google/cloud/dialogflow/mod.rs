#[cfg(
    any(
        feature = "google-cloud-dialogflow-cx-v3",
        feature = "google-cloud-dialogflow-cx-v3beta1",
    )
)]
pub mod cx;
#[cfg(any(feature = "google-cloud-dialogflow-v2"))]
pub mod v2;
#[cfg(any(feature = "google-cloud-dialogflow-v2beta1"))]
pub mod v2beta1;
