#[cfg(any(feature = "google-apps-alertcenter-v1beta1",))]
pub mod alertcenter;

#[cfg(any(feature = "google-apps-drive-activity-v2",))]
pub mod drive;

#[cfg(any(
    feature = "google-apps-script-type",
    feature = "google-apps-script-type-calendar",
    feature = "google-apps-script-type-docs",
    feature = "google-apps-script-type-drive",
    feature = "google-apps-script-type-gmail",
    feature = "google-apps-script-type-sheets",
    feature = "google-apps-script-type-slides",
))]
pub mod script;
