#[cfg(any(feature = "google-maps-addressvalidation-v1"))]
pub mod addressvalidation;
#[cfg(any(feature = "google-maps-aerialview-v1"))]
pub mod aerialview;
#[cfg(
    any(
        feature = "google-maps-mapsplatformdatasets-v1",
        feature = "google-maps-mapsplatformdatasets-v1alpha",
    )
)]
pub mod mapsplatformdatasets;
#[cfg(any(feature = "google-maps-mobilitybilling-logs-v1"))]
pub mod mobilitybilling;
#[cfg(any(feature = "google-maps-places-v1"))]
pub mod places;
#[cfg(
    any(
        feature = "google-maps-playablelocations-v3",
        feature = "google-maps-playablelocations-v3-sample",
    )
)]
pub mod playablelocations;
#[cfg(any(feature = "google-maps-regionlookup-v1alpha"))]
pub mod regionlookup;
#[cfg(any(feature = "google-maps-roads-v1op"))]
pub mod roads;
#[cfg(any(feature = "google-maps-routes-v1", feature = "google-maps-routes-v1alpha"))]
pub mod routes;
#[cfg(any(feature = "google-maps-routing-v2"))]
pub mod routing;
#[cfg(any(feature = "google-maps-solar-v1"))]
pub mod solar;
#[cfg(any(feature = "google-maps-unity"))]
pub mod unity;
