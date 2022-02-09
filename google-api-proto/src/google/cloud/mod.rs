#[cfg(any(feature = "google-cloud-accessapproval-v1",))]
pub mod accessapproval;

#[cfg(any(
    feature = "google-cloud-aiplatform-logging",
    feature = "google-cloud-aiplatform-v1",
    feature = "google-cloud-aiplatform-v1-schema-predict-instance",
    feature = "google-cloud-aiplatform-v1-schema-predict-params",
    feature = "google-cloud-aiplatform-v1-schema-predict-prediction",
    feature = "google-cloud-aiplatform-v1-schema-trainingjob-definition",
    feature = "google-cloud-aiplatform-v1beta1",
    feature = "google-cloud-aiplatform-v1beta1-schema",
    feature = "google-cloud-aiplatform-v1beta1-schema-predict-instance",
    feature = "google-cloud-aiplatform-v1beta1-schema-predict-params",
    feature = "google-cloud-aiplatform-v1beta1-schema-predict-prediction",
    feature = "google-cloud-aiplatform-v1beta1-schema-trainingjob-definition",
))]
pub mod aiplatform;

#[cfg(any(feature = "google-cloud-apigateway-v1",))]
pub mod apigateway;

#[cfg(any(feature = "google-cloud-apigeeconnect-v1",))]
pub mod apigeeconnect;

#[cfg(any(
    feature = "google-cloud-asset-v1",
    feature = "google-cloud-asset-v1p1beta1",
    feature = "google-cloud-asset-v1p2beta1",
    feature = "google-cloud-asset-v1p4beta1",
    feature = "google-cloud-asset-v1p5beta1",
    feature = "google-cloud-asset-v1p7beta1",
))]
pub mod asset;

#[cfg(any(
    feature = "google-cloud-assuredworkloads-v1",
    feature = "google-cloud-assuredworkloads-v1beta1",
))]
pub mod assuredworkloads;

#[cfg(any(feature = "google-cloud-audit",))]
pub mod audit;

#[cfg(any(
    feature = "google-cloud-automl-v1",
    feature = "google-cloud-automl-v1beta1",
))]
pub mod automl;

#[cfg(any(feature = "google-cloud-baremetalsolution-v2",))]
pub mod baremetalsolution;

#[cfg(any(
    feature = "google-cloud-bigquery-connection-v1",
    feature = "google-cloud-bigquery-connection-v1beta1",
    feature = "google-cloud-bigquery-datatransfer-v1",
    feature = "google-cloud-bigquery-logging-v1",
    feature = "google-cloud-bigquery-migration-v2alpha",
    feature = "google-cloud-bigquery-reservation-v1",
    feature = "google-cloud-bigquery-reservation-v1beta1",
    feature = "google-cloud-bigquery-storage-v1",
    feature = "google-cloud-bigquery-storage-v1beta1",
    feature = "google-cloud-bigquery-storage-v1beta2",
    feature = "google-cloud-bigquery-v2",
))]
pub mod bigquery;

#[cfg(any(
    feature = "google-cloud-billing-budgets-v1",
    feature = "google-cloud-billing-budgets-v1beta1",
    feature = "google-cloud-billing-v1",
))]
pub mod billing;

#[cfg(any(
    feature = "google-cloud-binaryauthorization-v1",
    feature = "google-cloud-binaryauthorization-v1beta1",
))]
pub mod binaryauthorization;

#[cfg(any(feature = "google-cloud-certificatemanager-v1",))]
pub mod certificatemanager;

#[cfg(any(feature = "google-cloud-channel-v1",))]
pub mod channel;

#[cfg(any(
    feature = "google-cloud-clouddms-logging-v1",
    feature = "google-cloud-clouddms-v1",
))]
pub mod clouddms;

#[cfg(any(feature = "google-cloud-common",))]
pub mod common;

#[cfg(any(
    feature = "google-cloud-compute-v1",
    feature = "google-cloud-compute-v1small",
))]
pub mod compute;

#[cfg(any(feature = "google-cloud-contactcenterinsights-v1",))]
pub mod contactcenterinsights;

#[cfg(any(
    feature = "google-cloud-datacatalog-v1",
    feature = "google-cloud-datacatalog-v1beta1",
))]
pub mod datacatalog;

#[cfg(any(
    feature = "google-cloud-datafusion-v1",
    feature = "google-cloud-datafusion-v1beta1",
))]
pub mod datafusion;

#[cfg(any(feature = "google-cloud-datalabeling-v1beta1",))]
pub mod datalabeling;

#[cfg(any(feature = "google-cloud-dataplex-v1",))]
pub mod dataplex;

#[cfg(any(
    feature = "google-cloud-dataproc-logging",
    feature = "google-cloud-dataproc-v1",
))]
pub mod dataproc;

#[cfg(any(feature = "google-cloud-dataqna-v1alpha",))]
pub mod dataqna;

#[cfg(any(
    feature = "google-cloud-datastream-v1",
    feature = "google-cloud-datastream-v1alpha1",
))]
pub mod datastream;

#[cfg(any(feature = "google-cloud-deploy-v1",))]
pub mod deploy;

#[cfg(any(
    feature = "google-cloud-dialogflow-cx-v3",
    feature = "google-cloud-dialogflow-cx-v3beta1",
    feature = "google-cloud-dialogflow-v2",
    feature = "google-cloud-dialogflow-v2beta1",
))]
pub mod dialogflow;

#[cfg(any(
    feature = "google-cloud-documentai-v1",
    feature = "google-cloud-documentai-v1beta1",
    feature = "google-cloud-documentai-v1beta2",
    feature = "google-cloud-documentai-v1beta3",
))]
pub mod documentai;

#[cfg(any(
    feature = "google-cloud-domains-v1",
    feature = "google-cloud-domains-v1alpha2",
    feature = "google-cloud-domains-v1beta1",
))]
pub mod domains;

#[cfg(any(feature = "google-cloud-essentialcontacts-v1",))]
pub mod essentialcontacts;

#[cfg(any(
    feature = "google-cloud-eventarc-publishing-v1",
    feature = "google-cloud-eventarc-v1",
))]
pub mod eventarc;

#[cfg(any(
    feature = "google-cloud-filestore-v1",
    feature = "google-cloud-filestore-v1beta1",
))]
pub mod filestore;

#[cfg(any(feature = "google-cloud-functions-v1",))]
pub mod functions;

#[cfg(any(
    feature = "google-cloud-gaming-allocationendpoint-v1alpha",
    feature = "google-cloud-gaming-v1",
    feature = "google-cloud-gaming-v1beta",
))]
pub mod gaming;

#[cfg(any(feature = "google-cloud-gkebackup-logging-v1",))]
pub mod gkebackup;

#[cfg(any(
    feature = "google-cloud-gkeconnect-gateway-v1",
    feature = "google-cloud-gkeconnect-gateway-v1alpha1",
    feature = "google-cloud-gkeconnect-gateway-v1beta1",
))]
pub mod gkeconnect;

#[cfg(any(
    feature = "google-cloud-gkehub-cloudauditlogging-v1alpha",
    feature = "google-cloud-gkehub-configmanagement-v1",
    feature = "google-cloud-gkehub-configmanagement-v1alpha",
    feature = "google-cloud-gkehub-configmanagement-v1beta",
    feature = "google-cloud-gkehub-metering-v1alpha",
    feature = "google-cloud-gkehub-metering-v1beta",
    feature = "google-cloud-gkehub-multiclusteringress-v1",
    feature = "google-cloud-gkehub-multiclusteringress-v1alpha",
    feature = "google-cloud-gkehub-multiclusteringress-v1beta",
    feature = "google-cloud-gkehub-servicemesh-v1alpha",
    feature = "google-cloud-gkehub-v1",
    feature = "google-cloud-gkehub-v1alpha",
    feature = "google-cloud-gkehub-v1alpha2",
    feature = "google-cloud-gkehub-v1beta",
    feature = "google-cloud-gkehub-v1beta1",
))]
pub mod gkehub;

#[cfg(any(
    feature = "google-cloud-gsuiteaddons-logging-v1",
    feature = "google-cloud-gsuiteaddons-v1",
))]
pub mod gsuiteaddons;

#[cfg(any(feature = "google-cloud-iap-v1", feature = "google-cloud-iap-v1beta1",))]
pub mod iap;

#[cfg(any(feature = "google-cloud-identitytoolkit-v2",))]
pub mod identitytoolkit;

#[cfg(any(feature = "google-cloud-ids-v1",))]
pub mod ids;

#[cfg(any(feature = "google-cloud-integrations-v1alpha",))]
pub mod integrations;

#[cfg(any(feature = "google-cloud-iot-v1",))]
pub mod iot;

#[cfg(any(feature = "google-cloud-kms-v1",))]
pub mod kms;

#[cfg(any(
    feature = "google-cloud-language-v1",
    feature = "google-cloud-language-v1beta1",
    feature = "google-cloud-language-v1beta2",
))]
pub mod language;

#[cfg(any(feature = "google-cloud-lifesciences-v2beta",))]
pub mod lifesciences;

#[cfg(any(feature = "google-cloud-location",))]
pub mod location;

#[cfg(any(
    feature = "google-cloud-managedidentities-v1",
    feature = "google-cloud-managedidentities-v1beta1",
))]
pub mod managedidentities;

#[cfg(any(
    feature = "google-cloud-mediatranslation-v1alpha1",
    feature = "google-cloud-mediatranslation-v1beta1",
))]
pub mod mediatranslation;

#[cfg(any(
    feature = "google-cloud-memcache-v1",
    feature = "google-cloud-memcache-v1beta2",
))]
pub mod memcache;

#[cfg(any(
    feature = "google-cloud-metastore-logging-v1",
    feature = "google-cloud-metastore-v1",
    feature = "google-cloud-metastore-v1alpha",
    feature = "google-cloud-metastore-v1beta",
))]
pub mod metastore;

#[cfg(any(feature = "google-cloud-ml-v1",))]
pub mod ml;

#[cfg(any(
    feature = "google-cloud-networkconnectivity-v1",
    feature = "google-cloud-networkconnectivity-v1alpha1",
))]
pub mod networkconnectivity;

#[cfg(any(
    feature = "google-cloud-networkmanagement-v1",
    feature = "google-cloud-networkmanagement-v1beta1",
))]
pub mod networkmanagement;

#[cfg(any(feature = "google-cloud-networksecurity-v1beta1",))]
pub mod networksecurity;

#[cfg(any(
    feature = "google-cloud-networkservices-v1",
    feature = "google-cloud-networkservices-v1beta1",
))]
pub mod networkservices;

#[cfg(any(
    feature = "google-cloud-notebooks-v1",
    feature = "google-cloud-notebooks-v1beta1",
))]
pub mod notebooks;

#[cfg(any(
    feature = "google-cloud-orchestration-airflow-service-v1",
    feature = "google-cloud-orchestration-airflow-service-v1beta1",
))]
pub mod orchestration;

#[cfg(any(
    feature = "google-cloud-orgpolicy-v1",
    feature = "google-cloud-orgpolicy-v2",
))]
pub mod orgpolicy;

#[cfg(any(
    feature = "google-cloud-osconfig-agentendpoint-v1",
    feature = "google-cloud-osconfig-agentendpoint-v1beta",
    feature = "google-cloud-osconfig-v1",
    feature = "google-cloud-osconfig-v1alpha",
    feature = "google-cloud-osconfig-v1beta",
))]
pub mod osconfig;

#[cfg(any(
    feature = "google-cloud-oslogin-common",
    feature = "google-cloud-oslogin-v1",
    feature = "google-cloud-oslogin-v1alpha",
    feature = "google-cloud-oslogin-v1beta",
))]
pub mod oslogin;

#[cfg(any(feature = "google-cloud-phishingprotection-v1beta1",))]
pub mod phishingprotection;

#[cfg(any(feature = "google-cloud-policytroubleshooter-v1",))]
pub mod policytroubleshooter;

#[cfg(any(feature = "google-cloud-privatecatalog-v1beta1",))]
pub mod privatecatalog;

#[cfg(any(feature = "google-cloud-pubsublite-v1",))]
pub mod pubsublite;

#[cfg(any(
    feature = "google-cloud-recaptchaenterprise-v1",
    feature = "google-cloud-recaptchaenterprise-v1beta1",
))]
pub mod recaptchaenterprise;

#[cfg(any(feature = "google-cloud-recommendationengine-v1beta1",))]
pub mod recommendationengine;

#[cfg(any(
    feature = "google-cloud-recommender-logging-v1",
    feature = "google-cloud-recommender-logging-v1beta1",
    feature = "google-cloud-recommender-v1",
    feature = "google-cloud-recommender-v1beta1",
))]
pub mod recommender;

#[cfg(any(
    feature = "google-cloud-redis-v1",
    feature = "google-cloud-redis-v1beta1",
))]
pub mod redis;

#[cfg(any(
    feature = "google-cloud-resourcemanager-v2",
    feature = "google-cloud-resourcemanager-v3",
))]
pub mod resourcemanager;

#[cfg(any(feature = "google-cloud-resourcesettings-v1",))]
pub mod resourcesettings;

#[cfg(any(
    feature = "google-cloud-retail-logging",
    feature = "google-cloud-retail-v2",
    feature = "google-cloud-retail-v2alpha",
    feature = "google-cloud-retail-v2beta",
))]
pub mod retail;

#[cfg(any(feature = "google-cloud-runtimeconfig-v1beta1",))]
pub mod runtimeconfig;

#[cfg(any(feature = "google-cloud-saasaccelerator-management-logs-v1",))]
pub mod saasaccelerator;

#[cfg(any(
    feature = "google-cloud-scheduler-v1",
    feature = "google-cloud-scheduler-v1beta1",
))]
pub mod scheduler;

#[cfg(any(
    feature = "google-cloud-secretmanager-logging-v1",
    feature = "google-cloud-secretmanager-v1",
))]
pub mod secretmanager;

#[cfg(any(feature = "google-cloud-secrets-v1beta1",))]
pub mod secrets;

#[cfg(any(
    feature = "google-cloud-security-privateca-v1",
    feature = "google-cloud-security-privateca-v1beta1",
))]
pub mod security;

#[cfg(any(
    feature = "google-cloud-securitycenter-settings-v1beta1",
    feature = "google-cloud-securitycenter-v1",
    feature = "google-cloud-securitycenter-v1beta1",
    feature = "google-cloud-securitycenter-v1p1beta1",
))]
pub mod securitycenter;

#[cfg(any(
    feature = "google-cloud-servicedirectory-v1",
    feature = "google-cloud-servicedirectory-v1beta1",
))]
pub mod servicedirectory;

#[cfg(any(feature = "google-cloud-shell-v1",))]
pub mod shell;

#[cfg(any(
    feature = "google-cloud-speech-v1",
    feature = "google-cloud-speech-v1p1beta1",
))]
pub mod speech;

#[cfg(any(feature = "google-cloud-sql-v1", feature = "google-cloud-sql-v1beta4",))]
pub mod sql;

#[cfg(any(
    feature = "google-cloud-support-common",
    feature = "google-cloud-support-v1alpha1",
))]
pub mod support;

#[cfg(any(
    feature = "google-cloud-talent-v4",
    feature = "google-cloud-talent-v4beta1",
))]
pub mod talent;

#[cfg(any(
    feature = "google-cloud-tasks-v2",
    feature = "google-cloud-tasks-v2beta2",
    feature = "google-cloud-tasks-v2beta3",
))]
pub mod tasks;

#[cfg(any(
    feature = "google-cloud-texttospeech-v1",
    feature = "google-cloud-texttospeech-v1beta1",
))]
pub mod texttospeech;

#[cfg(any(feature = "google-cloud-tpu-v1", feature = "google-cloud-tpu-v2alpha1",))]
pub mod tpu;

#[cfg(any(
    feature = "google-cloud-translation-v3",
    feature = "google-cloud-translation-v3beta1",
))]
pub mod translation;

#[cfg(any(
    feature = "google-cloud-video-livestream-logging-v1",
    feature = "google-cloud-video-livestream-v1",
    feature = "google-cloud-video-transcoder-v1",
))]
pub mod video;

#[cfg(any(
    feature = "google-cloud-videointelligence-v1",
    feature = "google-cloud-videointelligence-v1beta2",
    feature = "google-cloud-videointelligence-v1p1beta1",
    feature = "google-cloud-videointelligence-v1p2beta1",
    feature = "google-cloud-videointelligence-v1p3beta1",
))]
pub mod videointelligence;

#[cfg(any(
    feature = "google-cloud-vision-v1",
    feature = "google-cloud-vision-v1p1beta1",
    feature = "google-cloud-vision-v1p2beta1",
    feature = "google-cloud-vision-v1p3beta1",
    feature = "google-cloud-vision-v1p4beta1",
))]
pub mod vision;

#[cfg(any(feature = "google-cloud-vmmigration-v1",))]
pub mod vmmigration;

#[cfg(any(feature = "google-cloud-vpcaccess-v1",))]
pub mod vpcaccess;

#[cfg(any(
    feature = "google-cloud-webrisk-v1",
    feature = "google-cloud-webrisk-v1beta1",
))]
pub mod webrisk;

#[cfg(any(
    feature = "google-cloud-websecurityscanner-v1",
    feature = "google-cloud-websecurityscanner-v1alpha",
    feature = "google-cloud-websecurityscanner-v1beta",
))]
pub mod websecurityscanner;

#[cfg(any(
    feature = "google-cloud-workflows-executions-v1",
    feature = "google-cloud-workflows-executions-v1beta",
    feature = "google-cloud-workflows-type",
    feature = "google-cloud-workflows-v1",
    feature = "google-cloud-workflows-v1beta",
))]
pub mod workflows;

/// An enum to be used to mark the essential (for polling) fields in an
/// API-specific Operation object. A custom Operation object may contain many
/// different fields, but only few of them are essential to conduct a successful
/// polling process.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationResponseMapping {
    /// Do not use.
    Undefined = 0,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.name.
    Name = 1,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.done. If the annotated field is of
    /// an enum type, `annotated_field_name == EnumType.DONE` semantics should be
    /// equivalent to `Operation.done == true`. If the annotated field is of type
    /// boolean, then it should follow the same semantics as Operation.done.
    /// Otherwise, a non-empty value should be treated as `Operation.done == true`.
    Status = 2,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.error.code.
    ErrorCode = 3,
    /// A field in an API-specific (custom) Operation object which carries the same
    /// meaning as google.longrunning.Operation.error.message.
    ErrorMessage = 4,
}
