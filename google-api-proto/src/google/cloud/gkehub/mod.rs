#[cfg(any(feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",))]
pub mod cloudauditlogging;

#[cfg(any(
    feature = "google-cloud-gkehub-configmanagement-v1",
    feature = "google-cloud-gkehub-configmanagement-v1alpha",
    feature = "google-cloud-gkehub-configmanagement-v1beta",
))]
pub mod configmanagement;

#[cfg(any(
    feature = "google-cloud-gkehub-metering-v1alpha",
    feature = "google-cloud-gkehub-metering-v1beta",
))]
pub mod metering;

#[cfg(any(
    feature = "google-cloud-gkehub-multiclusteringress-v1",
    feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
    feature = "google-cloud-gkehub-multiclusteringress-v1beta",
))]
pub mod multiclusteringress;

#[cfg(any(feature = "google-cloud-gkehub-servicemesh-v1alpha",))]
pub mod servicemesh;

#[cfg(any(feature = "google-cloud-gkehub-v1",))]
pub mod v1;

#[cfg(any(feature = "google-cloud-gkehub-v1alpha",))]
pub mod v1alpha;

#[cfg(any(feature = "google-cloud-gkehub-v1alpha2",))]
pub mod v1alpha2;

#[cfg(any(feature = "google-cloud-gkehub-v1beta",))]
pub mod v1beta;

#[cfg(any(feature = "google-cloud-gkehub-v1beta1",))]
pub mod v1beta1;
