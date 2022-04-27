#[cfg(
    any(
        feature = "google-apps-script-type",
        feature = "google-apps-script-type-calendar",
        feature = "google-apps-script-type-docs",
        feature = "google-apps-script-type-drive",
        feature = "google-apps-script-type-gmail",
        feature = "google-apps-script-type-sheets",
        feature = "google-apps-script-type-slides",
    )
)]
pub mod r#type;
