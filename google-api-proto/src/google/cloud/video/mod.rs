#[cfg(any(feature = "google-cloud-video-livestream-v1",))]
pub mod livestream;

#[cfg(any(
    feature = "google-cloud-video-transcoder-v1",
    feature = "google-cloud-video-transcoder-v1beta1",
))]
pub mod transcoder;
