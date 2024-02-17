#[cfg(any(feature = "google-shopping-css-v1"))]
pub mod css;
#[cfg(
    any(
        feature = "google-shopping-merchant-inventories-v1beta",
        feature = "google-shopping-merchant-reports-v1beta",
    )
)]
pub mod merchant;
#[cfg(any(feature = "google-shopping-type"))]
pub mod r#type;
