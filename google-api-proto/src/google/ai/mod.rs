#[cfg(
    any(
        feature = "google-ai-generativelanguage-v1",
        feature = "google-ai-generativelanguage-v1beta",
        feature = "google-ai-generativelanguage-v1beta2",
        feature = "google-ai-generativelanguage-v1beta3",
    )
)]
pub mod generativelanguage;
