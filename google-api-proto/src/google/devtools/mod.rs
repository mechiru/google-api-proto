#[cfg(
    any(
        feature = "google-devtools-artifactregistry-v1",
        feature = "google-devtools-artifactregistry-v1beta2",
    )
)]
pub mod artifactregistry;
#[cfg(any(feature = "google-devtools-build-v1"))]
pub mod build;
#[cfg(any(feature = "google-devtools-cloudbuild-v1"))]
pub mod cloudbuild;
#[cfg(any(feature = "google-devtools-clouddebugger-v2"))]
pub mod clouddebugger;
#[cfg(any(feature = "google-devtools-clouderrorreporting-v1beta1"))]
pub mod clouderrorreporting;
#[cfg(any(feature = "google-devtools-cloudprofiler-v2"))]
pub mod cloudprofiler;
#[cfg(
    any(
        feature = "google-devtools-cloudtrace-v1",
        feature = "google-devtools-cloudtrace-v2",
    )
)]
pub mod cloudtrace;
#[cfg(
    any(
        feature = "google-devtools-containeranalysis-v1",
        feature = "google-devtools-containeranalysis-v1beta1",
    )
)]
pub mod containeranalysis;
#[cfg(any(feature = "google-devtools-remoteworkers-v1test2"))]
pub mod remoteworkers;
#[cfg(any(feature = "google-devtools-resultstore-v2"))]
pub mod resultstore;
#[cfg(any(feature = "google-devtools-source-v1"))]
pub mod source;
#[cfg(any(feature = "google-devtools-sourcerepo-v1"))]
pub mod sourcerepo;
#[cfg(any(feature = "google-devtools-testing-v1"))]
pub mod testing;
