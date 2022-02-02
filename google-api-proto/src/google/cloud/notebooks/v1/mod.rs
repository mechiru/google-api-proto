/// Definition of a software environment that is used to start a notebook
/// instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Output only. Name of this environment.
    /// Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Display name of this environment for the UI.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A brief description of this environment.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path. Example: `"gs://path-to-file/file-name"`
    #[prost(string, tag = "8")]
    pub post_startup_script: ::prost::alloc::string::String,
    /// Output only. The time at which this environment was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the environment; can be one of VM image, or container image.
    #[prost(oneof = "environment::ImageType", tags = "6, 7")]
    pub image_type: ::core::option::Option<environment::ImageType>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Type of the environment; can be one of VM image, or container image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ImageType {
        /// Use a Compute Engine VM image to start the notebook instance.
        #[prost(message, tag = "6")]
        VmImage(super::VmImage),
        /// Use a container image to start the notebook instance.
        #[prost(message, tag = "7")]
        ContainerImage(super::ContainerImage),
    }
}
/// Definition of a custom Compute Engine virtual machine image for starting a
/// notebook instance with the environment installed directly on the VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmImage {
    /// Required. The name of the Google Cloud project that this VM image belongs to.
    /// Format: `projects/{project_id}`
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// The reference to an external Compute Engine VM image.
    #[prost(oneof = "vm_image::Image", tags = "2, 3")]
    pub image: ::core::option::Option<vm_image::Image>,
}
/// Nested message and enum types in `VmImage`.
pub mod vm_image {
    /// The reference to an external Compute Engine VM image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Use VM image name to find the image.
        #[prost(string, tag = "2")]
        ImageName(::prost::alloc::string::String),
        /// Use this VM image family to find the image; the newest image in this
        /// family will be used.
        #[prost(string, tag = "3")]
        ImageFamily(::prost::alloc::string::String),
    }
}
/// Definition of a container image for starting a notebook instance with the
/// environment installed in a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerImage {
    /// Required. The path to the container image repository. For example:
    /// `gcr.io/{project_id}/{image_name}`
    #[prost(string, tag = "1")]
    pub repository: ::prost::alloc::string::String,
    /// The tag of the container image. If not specified, this defaults
    /// to the latest tag.
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// The description a notebook execution workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionTemplate {
    /// Required. Scale tier of the hardware used for notebook execution.
    /// DEPRECATED Will be discontinued. As right now only CUSTOM is supported.
    #[deprecated]
    #[prost(enumeration = "execution_template::ScaleTier", tag = "1")]
    pub scale_tier: i32,
    /// Specifies the type of virtual machine to use for your training
    /// job's master worker. You must specify this field when `scaleTier` is set to
    /// `CUSTOM`.
    ///
    /// You can use certain Compute Engine machine types directly in this field.
    /// The following types are supported:
    ///
    /// - `n1-standard-4`
    /// - `n1-standard-8`
    /// - `n1-standard-16`
    /// - `n1-standard-32`
    /// - `n1-standard-64`
    /// - `n1-standard-96`
    /// - `n1-highmem-2`
    /// - `n1-highmem-4`
    /// - `n1-highmem-8`
    /// - `n1-highmem-16`
    /// - `n1-highmem-32`
    /// - `n1-highmem-64`
    /// - `n1-highmem-96`
    /// - `n1-highcpu-16`
    /// - `n1-highcpu-32`
    /// - `n1-highcpu-64`
    /// - `n1-highcpu-96`
    ///
    ///
    /// Alternatively, you can use the following legacy machine types:
    ///
    /// - `standard`
    /// - `large_model`
    /// - `complex_model_s`
    /// - `complex_model_m`
    /// - `complex_model_l`
    /// - `standard_gpu`
    /// - `complex_model_m_gpu`
    /// - `complex_model_l_gpu`
    /// - `standard_p100`
    /// - `complex_model_m_p100`
    /// - `standard_v100`
    /// - `large_model_v100`
    /// - `complex_model_m_v100`
    /// - `complex_model_l_v100`
    ///
    ///
    /// Finally, if you want to use a TPU for training, specify `cloud_tpu` in this
    /// field. Learn more about the [special configuration options for training
    /// with TPU.
    #[prost(string, tag = "2")]
    pub master_type: ::prost::alloc::string::String,
    /// Configuration (count and accelerator type) for hardware running notebook
    /// execution.
    #[prost(message, optional, tag = "3")]
    pub accelerator_config: ::core::option::Option<execution_template::SchedulerAcceleratorConfig>,
    /// Labels for execution.
    /// If execution is scheduled, a field included will be 'nbs-scheduled'.
    /// Otherwise, it is an immediate execution, and an included field will be
    /// 'nbs-immediate'. Use fields to efficiently index between various types of
    /// executions.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Path to the notebook file to execute.
    /// Must be in a Google Cloud Storage bucket.
    /// Format: `gs://{project_id}/{folder}/{notebook_file_name}`
    /// Ex: `gs://notebook_user/scheduled_notebooks/sentiment_notebook.ipynb`
    #[prost(string, tag = "5")]
    pub input_notebook_file: ::prost::alloc::string::String,
    /// Container Image URI to a DLVM
    /// Example: 'gcr.io/deeplearning-platform-release/base-cu100'
    /// More examples can be found at:
    /// <https://cloud.google.com/ai-platform/deep-learning-containers/docs/choosing-container>
    #[prost(string, tag = "6")]
    pub container_image_uri: ::prost::alloc::string::String,
    /// Path to the notebook folder to write to.
    /// Must be in a Google Cloud Storage bucket path.
    /// Format: `gs://{project_id}/{folder}`
    /// Ex: `gs://notebook_user/scheduled_notebooks`
    #[prost(string, tag = "7")]
    pub output_notebook_folder: ::prost::alloc::string::String,
    /// Parameters to be overridden in the notebook during execution.
    /// Ref <https://papermill.readthedocs.io/en/latest/usage-parameterize.html> on
    /// how to specifying parameters in the input notebook and pass them here
    /// in an YAML file.
    /// Ex: `gs://notebook_user/scheduled_notebooks/sentiment_notebook_params.yaml`
    #[prost(string, tag = "8")]
    pub params_yaml_file: ::prost::alloc::string::String,
    /// Parameters used within the 'input_notebook_file' notebook.
    #[prost(string, tag = "9")]
    pub parameters: ::prost::alloc::string::String,
    /// The email address of a service account to use when running the execution.
    /// You must have the `iam.serviceAccounts.actAs` permission for the specified
    /// service account.
    #[prost(string, tag = "10")]
    pub service_account: ::prost::alloc::string::String,
    /// The type of Job to be used on this execution.
    #[prost(enumeration = "execution_template::JobType", tag = "11")]
    pub job_type: i32,
    /// Parameters for an execution type.
    /// NOTE: There are currently no extra parameters for VertexAI jobs.
    #[prost(oneof = "execution_template::JobParameters", tags = "12")]
    pub job_parameters: ::core::option::Option<execution_template::JobParameters>,
}
/// Nested message and enum types in `ExecutionTemplate`.
pub mod execution_template {
    /// Definition of a hardware accelerator. Note that not all combinations
    /// of `type` and `core_count` are valid. Check GPUs on
    /// Compute Engine to find a valid
    /// combination. TPUs are not supported.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchedulerAcceleratorConfig {
        /// Type of this accelerator.
        #[prost(enumeration = "SchedulerAcceleratorType", tag = "1")]
        pub r#type: i32,
        /// Count of cores of this accelerator.
        #[prost(int64, tag = "2")]
        pub core_count: i64,
    }
    /// Parameters used in Dataproc JobType executions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataprocParameters {
        /// URI for cluster used to run Dataproc execution.
        /// Format: `projects/{PROJECT_ID}/regions/{REGION}/clusters/{CLUSTER_NAME}`
        #[prost(string, tag = "1")]
        pub cluster: ::prost::alloc::string::String,
    }
    /// Required. Specifies the machine types, the number of replicas for workers
    /// and parameter servers.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ScaleTier {
        /// Unspecified Scale Tier.
        Unspecified = 0,
        /// A single worker instance. This tier is suitable for learning how to use
        /// Cloud ML, and for experimenting with new models using small datasets.
        Basic = 1,
        /// Many workers and a few parameter servers.
        Standard1 = 2,
        /// A large number of workers with many parameter servers.
        Premium1 = 3,
        /// A single worker instance with a K80 GPU.
        BasicGpu = 4,
        /// A single worker instance with a Cloud TPU.
        BasicTpu = 5,
        /// The CUSTOM tier is not a set tier, but rather enables you to use your
        /// own cluster specification. When you use this tier, set values to
        /// configure your processing cluster according to these guidelines:
        ///
        /// *   You _must_ set `TrainingInput.masterType` to specify the type
        ///     of machine to use for your master node. This is the only required
        ///     setting.
        ///
        /// *   You _may_ set `TrainingInput.workerCount` to specify the number of
        ///     workers to use. If you specify one or more workers, you _must_ also
        ///     set `TrainingInput.workerType` to specify the type of machine to use
        ///     for your worker nodes.
        ///
        /// *   You _may_ set `TrainingInput.parameterServerCount` to specify the
        ///     number of parameter servers to use. If you specify one or more
        ///     parameter servers, you _must_ also set
        ///     `TrainingInput.parameterServerType` to specify the type of machine to
        ///     use for your parameter servers.
        ///
        /// Note that all of your workers must use the same machine type, which can
        /// be different from your parameter server type and master type. Your
        /// parameter servers must likewise use the same machine type, which can be
        /// different from your worker type and master type.
        Custom = 6,
    }
    /// Hardware accelerator types for AI Platform Training jobs.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SchedulerAcceleratorType {
        /// Unspecified accelerator type. Default to no GPU.
        Unspecified = 0,
        /// Nvidia Tesla K80 GPU.
        NvidiaTeslaK80 = 1,
        /// Nvidia Tesla P100 GPU.
        NvidiaTeslaP100 = 2,
        /// Nvidia Tesla V100 GPU.
        NvidiaTeslaV100 = 3,
        /// Nvidia Tesla P4 GPU.
        NvidiaTeslaP4 = 4,
        /// Nvidia Tesla T4 GPU.
        NvidiaTeslaT4 = 5,
        /// TPU v2.
        TpuV2 = 6,
        /// TPU v3.
        TpuV3 = 7,
    }
    /// The backend used for this execution.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobType {
        /// No type specified.
        Unspecified = 0,
        /// Custom Job in `aiplatform.googleapis.com`.
        /// Default value for an execution.
        VertexAi = 1,
        /// Run execution on a cluster with Dataproc as a job.
        /// <https://cloud.google.com/dataproc/docs/reference/rest/v1/projects.regions.jobs>
        Dataproc = 2,
    }
    /// Parameters for an execution type.
    /// NOTE: There are currently no extra parameters for VertexAI jobs.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JobParameters {
        /// Parameters used in Dataproc JobType executions.
        #[prost(message, tag = "12")]
        DataprocParameters(DataprocParameters),
    }
}
/// The definition of a single executed notebook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Execution {
    /// execute metadata including name, hardware spec, region, labels, etc.
    #[prost(message, optional, tag = "1")]
    pub execution_template: ::core::option::Option<ExecutionTemplate>,
    /// Output only. The resource name of the execute. Format:
    /// `projects/{project_id}/locations/{location}/execution/{execution_id}`
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Name used for UI purposes.
    /// Name can only contain alphanumeric characters and underscores '_'.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// A brief description of this execution.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time the Execution was instantiated.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the Execution was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. State of the underlying AI Platform job.
    #[prost(enumeration = "execution::State", tag = "7")]
    pub state: i32,
    /// Output notebook file generated by this execution
    #[prost(string, tag = "8")]
    pub output_notebook_file: ::prost::alloc::string::String,
    /// Output only. The URI of the external job used to execute the notebook.
    #[prost(string, tag = "9")]
    pub job_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Execution`.
pub mod execution {
    /// Enum description of the state of the underlying AIP job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The job state is unspecified.
        Unspecified = 0,
        /// The job has been just created and processing has not yet begun.
        Queued = 1,
        /// The service is preparing to execution the job.
        Preparing = 2,
        /// The job is in progress.
        Running = 3,
        /// The job completed successfully.
        Succeeded = 4,
        /// The job failed.
        /// `error_message` should contain the details of the failure.
        Failed = 5,
        /// The job is being cancelled.
        /// `error_message` should describe the reason for the cancellation.
        Cancelling = 6,
        /// The job has been cancelled.
        /// `error_message` should describe the reason for the cancellation.
        Cancelled = 7,
        /// The jobs has become expired (added for uCAIP jobs)
        /// <https://cloud.google.com/vertex-ai/docs/reference/rest/v1/JobState>
        Expired = 9,
        /// The Execution is being created.
        Initializing = 10,
    }
}
/// Reservation Affinity for consuming Zonal reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReservationAffinity {
    /// Optional. Type of reservation to consume
    #[prost(enumeration = "reservation_affinity::Type", tag = "1")]
    pub consume_reservation_type: i32,
    /// Optional. Corresponds to the label key of reservation resource.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Optional. Corresponds to the label values of reservation resource.
    #[prost(string, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ReservationAffinity`.
pub mod reservation_affinity {
    /// Indicates whether to consume capacity from an reservation or not.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default type.
        Unspecified = 0,
        /// Do not consume from any allocated capacity.
        NoReservation = 1,
        /// Consume any reservation available.
        AnyReservation = 2,
        /// Must consume from a specific reservation. Must specify key value fields
        /// for specifying the reservations.
        SpecificReservation = 3,
    }
}
/// The definition of a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The name of this notebook instance. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path (`gs://path-to-file/file-name`).
    #[prost(string, tag = "4")]
    pub post_startup_script: ::prost::alloc::string::String,
    /// Output only. The proxy endpoint that is used to access the Jupyter notebook.
    #[prost(string, tag = "5")]
    pub proxy_uri: ::prost::alloc::string::String,
    /// Input only. The owner of this instance after creation. Format: `alias@example.com`
    ///
    /// Currently supports one owner only. If not specified, all of the service
    /// account users of your VM instance's service account can use
    /// the instance.
    #[prost(string, repeated, tag = "6")]
    pub instance_owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The service account on this instance, giving access to other Google
    /// Cloud services.
    /// You can use any service account within the same project, but you
    /// must have the service account user permission to use the instance.
    ///
    /// If not specified, the [Compute Engine default service
    /// account](<https://cloud.google.com/compute/docs/access/service-accounts#default_service_account>)
    /// is used.
    #[prost(string, tag = "7")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. The URIs of service account scopes to be included in
    /// Compute Engine instances.
    ///
    /// If not specified, the following
    /// \[scopes\](<https://cloud.google.com/compute/docs/access/service-accounts#accesscopesiam>)
    /// are defined:
    ///  - <https://www.googleapis.com/auth/cloud-platform>
    ///  - <https://www.googleapis.com/auth/userinfo.email>
    /// If not using default scopes, you need at least:
    ///    <https://www.googleapis.com/auth/compute>
    #[prost(string, repeated, tag = "31")]
    pub service_account_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The [Compute Engine machine type](/compute/docs/machine-types) of this
    /// instance.
    #[prost(string, tag = "8")]
    pub machine_type: ::prost::alloc::string::String,
    /// The hardware accelerator used on this instance. If you use
    /// accelerators, make sure that your configuration has
    /// [enough vCPUs and memory to support the `machine_type` you
    /// have selected](/compute/docs/gpus/#gpus-list).
    #[prost(message, optional, tag = "9")]
    pub accelerator_config: ::core::option::Option<instance::AcceleratorConfig>,
    /// Output only. The state of this instance.
    #[prost(enumeration = "instance::State", tag = "10")]
    pub state: i32,
    /// Whether the end user authorizes Google Cloud to install GPU driver
    /// on this instance.
    /// If this field is empty or set to false, the GPU driver won't be installed.
    /// Only applicable to instances with GPUs.
    #[prost(bool, tag = "11")]
    pub install_gpu_driver: bool,
    /// Specify a custom Cloud Storage path where the GPU driver is stored.
    /// If not specified, we'll automatically choose from official GPU drivers.
    #[prost(string, tag = "12")]
    pub custom_gpu_driver_path: ::prost::alloc::string::String,
    /// Input only. The type of the boot disk attached to this instance, defaults to
    /// standard persistent disk (`PD_STANDARD`).
    #[prost(enumeration = "instance::DiskType", tag = "13")]
    pub boot_disk_type: i32,
    /// Input only. The size of the boot disk in GB attached to this instance, up to a maximum
    /// of 64000&nbsp;GB (64&nbsp;TB). The minimum recommended value is
    /// 100&nbsp;GB. If not specified, this defaults to 100.
    #[prost(int64, tag = "14")]
    pub boot_disk_size_gb: i64,
    /// Input only. The type of the data disk attached to this instance, defaults to
    /// standard persistent disk (`PD_STANDARD`).
    #[prost(enumeration = "instance::DiskType", tag = "25")]
    pub data_disk_type: i32,
    /// Input only. The size of the data disk in GB attached to this instance, up to a maximum
    /// of 64000&nbsp;GB (64&nbsp;TB). You can choose the size of the data disk
    /// based on how big your notebooks and data are. If not specified, this
    /// defaults to 100.
    #[prost(int64, tag = "26")]
    pub data_disk_size_gb: i64,
    /// Input only. If true, the data disk will not be auto deleted when deleting the instance.
    #[prost(bool, tag = "27")]
    pub no_remove_data_disk: bool,
    /// Input only. Disk encryption method used on the boot and data disks, defaults to GMEK.
    #[prost(enumeration = "instance::DiskEncryption", tag = "15")]
    pub disk_encryption: i32,
    /// Input only. The KMS key used to encrypt the disks, only applicable if disk_encryption
    /// is CMEK.
    /// Format:
    /// `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
    ///
    /// Learn more about [using your own encryption keys](/kms/docs/quickstart).
    #[prost(string, tag = "16")]
    pub kms_key: ::prost::alloc::string::String,
    /// Output only. Attached disks to notebook instance.
    #[prost(message, repeated, tag = "28")]
    pub disks: ::prost::alloc::vec::Vec<instance::Disk>,
    /// Optional. Shielded VM configuration.
    /// [Images using supported Shielded VM
    /// features](<https://cloud.google.com/compute/docs/instances/modifying-shielded-vm>).
    #[prost(message, optional, tag = "30")]
    pub shielded_instance_config: ::core::option::Option<instance::ShieldedInstanceConfig>,
    /// If true, no public IP will be assigned to this instance.
    #[prost(bool, tag = "17")]
    pub no_public_ip: bool,
    /// If true, the notebook instance will not register with the proxy.
    #[prost(bool, tag = "18")]
    pub no_proxy_access: bool,
    /// The name of the VPC that this instance is in.
    /// Format:
    /// `projects/{project_id}/global/networks/{network_id}`
    #[prost(string, tag = "19")]
    pub network: ::prost::alloc::string::String,
    /// The name of the subnet that this instance is in.
    /// Format:
    /// `projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}`
    #[prost(string, tag = "20")]
    pub subnet: ::prost::alloc::string::String,
    /// Labels to apply to this instance.
    /// These can be later modified by the setLabels method.
    #[prost(btree_map = "string, string", tag = "21")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Custom metadata to apply to this instance.
    #[prost(btree_map = "string, string", tag = "22")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The Compute Engine tags to add to runtime (see [Tagging
    /// instances](<https://cloud.google.com/compute/docs/label-or-tag-resources#tags>)).
    #[prost(string, repeated, tag = "32")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The upgrade history of this instance.
    #[prost(message, repeated, tag = "29")]
    pub upgrade_history: ::prost::alloc::vec::Vec<instance::UpgradeHistoryEntry>,
    /// Optional. The type of vNIC to be used on this interface. This may be gVNIC or
    /// VirtioNet.
    #[prost(enumeration = "instance::NicType", tag = "33")]
    pub nic_type: i32,
    /// Optional. The optional reservation affinity. Setting this field will apply
    /// the specified [Zonal Compute
    /// Reservation](<https://cloud.google.com/compute/docs/instances/reserving-zonal-resources>)
    /// to this notebook instance.
    #[prost(message, optional, tag = "34")]
    pub reservation_affinity: ::core::option::Option<ReservationAffinity>,
    /// Output only. Instance creation time.
    #[prost(message, optional, tag = "23")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Instance update time.
    #[prost(message, optional, tag = "24")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the environment; can be one of VM image, or container image.
    #[prost(oneof = "instance::Environment", tags = "2, 3")]
    pub environment: ::core::option::Option<instance::Environment>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Definition of a hardware accelerator. Note that not all combinations
    /// of `type` and `core_count` are valid. Check [GPUs on
    /// Compute Engine](/compute/docs/gpus/#gpus-list) to find a valid
    /// combination. TPUs are not supported.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AcceleratorConfig {
        /// Type of this accelerator.
        #[prost(enumeration = "AcceleratorType", tag = "1")]
        pub r#type: i32,
        /// Count of cores of this accelerator.
        #[prost(int64, tag = "2")]
        pub core_count: i64,
    }
    /// An instance-attached disk resource.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Disk {
        /// Indicates whether the disk will be auto-deleted when the instance is
        /// deleted (but not when the disk is detached from the instance).
        #[prost(bool, tag = "1")]
        pub auto_delete: bool,
        /// Indicates that this is a boot disk. The virtual machine will use the
        /// first partition of the disk for its root filesystem.
        #[prost(bool, tag = "2")]
        pub boot: bool,
        /// Indicates a unique device name of your choice that is reflected into the
        /// /dev/disk/by-id/google-* tree of a Linux operating system running within
        /// the instance. This name can be used to reference the device for mounting,
        /// resizing, and so on, from within the instance.
        ///
        /// If not specified, the server chooses a default device name to apply to
        /// this disk, in the form persistent-disk-x, where x is a number assigned by
        /// Google Compute Engine.This field is only applicable for persistent disks.
        #[prost(string, tag = "3")]
        pub device_name: ::prost::alloc::string::String,
        /// Indicates the size of the disk in base-2 GB.
        #[prost(int64, tag = "4")]
        pub disk_size_gb: i64,
        /// Indicates a list of features to enable on the guest operating system.
        /// Applicable only for bootable images. Read  Enabling guest operating
        /// system features to see a list of available options.
        #[prost(message, repeated, tag = "5")]
        pub guest_os_features: ::prost::alloc::vec::Vec<disk::GuestOsFeature>,
        /// A zero-based index to this disk, where 0 is reserved for the
        /// boot disk. If you have many disks attached to an instance, each disk
        /// would have a unique index number.
        #[prost(int64, tag = "6")]
        pub index: i64,
        /// Indicates the disk interface to use for attaching this disk, which is
        /// either SCSI or NVME. The default is SCSI. Persistent disks must always
        /// use SCSI and the request will fail if you attempt to attach a persistent
        /// disk in any other format than SCSI. Local SSDs can use either NVME or
        /// SCSI. For performance characteristics of SCSI over NVMe, see Local SSD
        /// performance.
        /// Valid values:
        ///
        /// * NVME
        /// * SCSI
        #[prost(string, tag = "7")]
        pub interface: ::prost::alloc::string::String,
        /// Type of the resource. Always compute#attachedDisk for attached
        /// disks.
        #[prost(string, tag = "8")]
        pub kind: ::prost::alloc::string::String,
        /// A list of publicly visible licenses. Reserved for Google's use.
        /// A License represents billing and aggregate usage data for public
        /// and marketplace images.
        #[prost(string, repeated, tag = "9")]
        pub licenses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The mode in which to attach this disk, either READ_WRITE or READ_ONLY. If
        /// not specified, the default is to attach the disk in READ_WRITE mode.
        /// Valid values:
        ///
        /// * READ_ONLY
        /// * READ_WRITE
        #[prost(string, tag = "10")]
        pub mode: ::prost::alloc::string::String,
        /// Indicates a valid partial or full URL to an existing Persistent Disk
        /// resource.
        #[prost(string, tag = "11")]
        pub source: ::prost::alloc::string::String,
        /// Indicates the type of the disk, either SCRATCH or PERSISTENT.
        /// Valid values:
        ///
        /// * PERSISTENT
        /// * SCRATCH
        #[prost(string, tag = "12")]
        pub r#type: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Disk`.
    pub mod disk {
        /// Guest OS features for boot disk.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GuestOsFeature {
            /// The ID of a supported feature. Read  Enabling guest operating system
            /// features to see a list of available options.
            /// Valid values:
            ///
            /// * FEATURE_TYPE_UNSPECIFIED
            /// * MULTI_IP_SUBNET
            /// * SECURE_BOOT
            /// * UEFI_COMPATIBLE
            /// * VIRTIO_SCSI_MULTIQUEUE
            /// * WINDOWS
            #[prost(string, tag = "1")]
            pub r#type: ::prost::alloc::string::String,
        }
    }
    /// A set of Shielded Instance options.
    /// Check [Images using supported Shielded VM features]
    /// Not all combinations are valid.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShieldedInstanceConfig {
        /// Defines whether the instance has Secure Boot enabled.
        ///
        /// Secure Boot helps ensure that the system only runs authentic software by
        /// verifying the digital signature of all boot components, and halting the
        /// boot process if signature verification fails. Disabled by default.
        #[prost(bool, tag = "1")]
        pub enable_secure_boot: bool,
        /// Defines whether the instance has the vTPM enabled. Enabled by default.
        #[prost(bool, tag = "2")]
        pub enable_vtpm: bool,
        /// Defines whether the instance has integrity monitoring enabled.
        ///
        /// Enables monitoring and attestation of the boot integrity of the instance.
        /// The attestation is performed against the integrity policy baseline. This
        /// baseline is initially derived from the implicitly trusted boot image when
        /// the instance is created. Enabled by default.
        #[prost(bool, tag = "3")]
        pub enable_integrity_monitoring: bool,
    }
    /// The entry of VM image upgrade history.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpgradeHistoryEntry {
        /// The snapshot of the boot disk of this notebook instance before upgrade.
        #[prost(string, tag = "1")]
        pub snapshot: ::prost::alloc::string::String,
        /// The VM image before this instance upgrade.
        #[prost(string, tag = "2")]
        pub vm_image: ::prost::alloc::string::String,
        /// The container image before this instance upgrade.
        #[prost(string, tag = "3")]
        pub container_image: ::prost::alloc::string::String,
        /// The framework of this notebook instance.
        #[prost(string, tag = "4")]
        pub framework: ::prost::alloc::string::String,
        /// The version of the notebook instance before this upgrade.
        #[prost(string, tag = "5")]
        pub version: ::prost::alloc::string::String,
        /// The state of this instance upgrade history entry.
        #[prost(enumeration = "upgrade_history_entry::State", tag = "6")]
        pub state: i32,
        /// The time that this instance upgrade history entry is created.
        #[prost(message, optional, tag = "7")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Target VM Image. Format: ainotebooks-vm/project/image-name/name.
        #[deprecated]
        #[prost(string, tag = "8")]
        pub target_image: ::prost::alloc::string::String,
        /// Action. Rolloback or Upgrade.
        #[prost(enumeration = "upgrade_history_entry::Action", tag = "9")]
        pub action: i32,
        /// Target VM Version, like m63.
        #[prost(string, tag = "10")]
        pub target_version: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `UpgradeHistoryEntry`.
    pub mod upgrade_history_entry {
        /// The definition of the states of this upgrade history entry.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum State {
            /// State is not specified.
            Unspecified = 0,
            /// The instance upgrade is started.
            Started = 1,
            /// The instance upgrade is succeeded.
            Succeeded = 2,
            /// The instance upgrade is failed.
            Failed = 3,
        }
        /// The definition of operations of this upgrade history entry.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Action {
            /// Operation is not specified.
            Unspecified = 0,
            /// Upgrade.
            Upgrade = 1,
            /// Rollback.
            Rollback = 2,
        }
    }
    /// Definition of the types of hardware accelerators that can be used on this
    /// instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AcceleratorType {
        /// Accelerator type is not specified.
        Unspecified = 0,
        /// Accelerator type is Nvidia Tesla K80.
        NvidiaTeslaK80 = 1,
        /// Accelerator type is Nvidia Tesla P100.
        NvidiaTeslaP100 = 2,
        /// Accelerator type is Nvidia Tesla V100.
        NvidiaTeslaV100 = 3,
        /// Accelerator type is Nvidia Tesla P4 GPU.
        NvidiaTeslaP4 = 4,
        /// Accelerator type is Nvidia Tesla T4.
        NvidiaTeslaT4 = 5,
        /// Accelerator type is Nvidia Tesla A100.
        NvidiaTeslaA100 = 11,
        /// Accelerator type is NVIDIA Tesla T4 Virtual Workstations.
        NvidiaTeslaT4Vws = 8,
        /// Accelerator type is NVIDIA Tesla P100 Virtual Workstations.
        NvidiaTeslaP100Vws = 9,
        /// Accelerator type is Nvidia Tesla P4 GPU Virtual Workstations.
        NvidiaTeslaP4Vws = 10,
        /// (Coming soon) Accelerator type is TPU V2.
        TpuV2 = 6,
        /// (Coming soon) Accelerator type is TPU V3.
        TpuV3 = 7,
    }
    /// The definition of the states of this instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State is not specified.
        Unspecified = 0,
        /// The control logic is starting the instance.
        Starting = 1,
        /// The control logic is installing required frameworks and registering the
        /// instance with notebook proxy
        Provisioning = 2,
        /// The instance is running.
        Active = 3,
        /// The control logic is stopping the instance.
        Stopping = 4,
        /// The instance is stopped.
        Stopped = 5,
        /// The instance is deleted.
        Deleted = 6,
        /// The instance is upgrading.
        Upgrading = 7,
        /// The instance is being created.
        Initializing = 8,
        /// The instance is getting registered.
        Registering = 9,
    }
    /// Possible disk types for notebook instances.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiskType {
        /// Disk type not set.
        Unspecified = 0,
        /// Standard persistent disk type.
        PdStandard = 1,
        /// SSD persistent disk type.
        PdSsd = 2,
        /// Balanced persistent disk type.
        PdBalanced = 3,
    }
    /// Definition of the disk encryption options.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiskEncryption {
        /// Disk encryption is not specified.
        Unspecified = 0,
        /// Use Google managed encryption keys to encrypt the boot disk.
        Gmek = 1,
        /// Use customer managed encryption keys to encrypt the boot disk.
        Cmek = 2,
    }
    /// The type of vNIC driver.
    /// Default should be UNSPECIFIED_NIC_TYPE.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NicType {
        /// No type specified.
        UnspecifiedNicType = 0,
        /// VIRTIO
        VirtioNet = 1,
        /// GVNIC
        Gvnic = 2,
    }
    /// Type of the environment; can be one of VM image, or container image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Environment {
        /// Use a Compute Engine VM image to start the notebook instance.
        #[prost(message, tag = "2")]
        VmImage(super::VmImage),
        /// Use a container image to start the notebook instance.
        #[prost(message, tag = "3")]
        ContainerImage(super::ContainerImage),
    }
}
/// Notebook instance configurations that can be updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceConfig {
    /// Cron expression in UTC timezone, used to schedule instance auto upgrade.
    /// Please follow the [cron format](<https://en.wikipedia.org/wiki/Cron>).
    #[prost(string, tag = "1")]
    pub notebook_upgrade_schedule: ::prost::alloc::string::String,
    /// Verifies core internal services are running.
    #[prost(bool, tag = "2")]
    pub enable_health_monitoring: bool,
}
/// The definition of a schedule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    /// Output only. The name of this schedule. Format:
    /// `projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Display name used for UI purposes.
    /// Name can only contain alphanumeric characters, hyphens '-',
    /// and underscores '_'.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A brief description of this environment.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "schedule::State", tag = "4")]
    pub state: i32,
    /// Cron-tab formatted schedule by which the job will execute
    /// Format: minute, hour, day of month, month, day of week
    /// e.g. 0 0 * * WED = every Wednesday
    /// More examples: <https://crontab.guru/examples.html>
    #[prost(string, tag = "5")]
    pub cron_schedule: ::prost::alloc::string::String,
    /// Timezone on which the cron_schedule.
    /// The value of this field must be a time zone name from the tz database.
    /// TZ Database: <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>
    ///
    /// Note that some time zones include a provision for daylight savings time.
    /// The rules for daylight saving time are determined by the chosen tz.
    /// For UTC use the string "utc". If a time zone is not specified,
    /// the default will be in UTC (also known as GMT).
    #[prost(string, tag = "6")]
    pub time_zone: ::prost::alloc::string::String,
    /// Output only. Time the schedule was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the schedule was last updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Notebook Execution Template corresponding to this schedule.
    #[prost(message, optional, tag = "9")]
    pub execution_template: ::core::option::Option<ExecutionTemplate>,
    /// Output only. The most recent execution names triggered from this schedule and their
    /// corresponding states.
    #[prost(message, repeated, tag = "10")]
    pub recent_executions: ::prost::alloc::vec::Vec<Execution>,
}
/// Nested message and enum types in `Schedule`.
pub mod schedule {
    /// State of the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The job is executing normally.
        Enabled = 1,
        /// The job is paused by the user. It will not execute. A user can
        /// intentionally pause the job using
        /// \[PauseJobRequest][\].
        Paused = 2,
        /// The job is disabled by the system due to error. The user
        /// cannot directly set a job to be disabled.
        Disabled = 3,
        /// The job state resulting from a failed \[CloudScheduler.UpdateJob][\]
        /// operation. To recover a job from this state, retry
        /// \[CloudScheduler.UpdateJob][\] until a successful response is received.
        UpdateFailed = 4,
        /// The schedule resource is being created.
        Initializing = 5,
        /// The schedule resource is being deleted.
        Deleting = 6,
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// API endpoint name of this operation.
    #[prost(string, tag = "8")]
    pub endpoint: ::prost::alloc::string::String,
}
/// Request for listing notebook instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing
    /// from the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing notebook instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of returned instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Page token that can be used to continue listing from the last result in the
    /// next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. For example,
    /// ['us-west1-a', 'us-central1-b'].
    /// A ListInstancesResponse will only contain either instances or unreachables,
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for creating a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this instance.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The instance to be created.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// Request for registering a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterInstanceRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User defined unique ID of this instance. The `instance_id` must
    /// be 1 to 63 characters long and contain only lowercase letters,
    /// numeric characters, and dashes. The first character must be a lowercase
    /// letter and the last character cannot be a dash.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
}
/// Request for setting instance accelerator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceAcceleratorRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Type of this accelerator.
    #[prost(enumeration = "instance::AcceleratorType", tag = "2")]
    pub r#type: i32,
    /// Required. Count of cores of this accelerator. Note that not all combinations
    /// of `type` and `core_count` are valid. Check [GPUs on
    /// Compute Engine](<https://cloud.google.com/compute/docs/gpus/#gpus-list>) to
    /// find a valid combination. TPUs are not supported.
    #[prost(int64, tag = "3")]
    pub core_count: i64,
}
/// Request for setting instance machine type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceMachineTypeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The [Compute Engine machine
    /// type](<https://cloud.google.com/compute/docs/machine-types>).
    #[prost(string, tag = "2")]
    pub machine_type: ::prost::alloc::string::String,
}
/// Request for updating instance configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceConfigRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The instance configurations to be updated.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<InstanceConfig>,
}
/// Request for setting instance labels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceLabelsRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Labels to apply to this instance.
    /// These can be later modified by the setLabels method
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Request for updating the Shielded Instance config for a notebook instance.
/// You can only use this method on a stopped instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShieldedInstanceConfigRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// ShieldedInstance configuration to be updated.
    #[prost(message, optional, tag = "2")]
    pub shielded_instance_config: ::core::option::Option<instance::ShieldedInstanceConfig>,
}
/// Request for deleting a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for starting a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for stopping a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for reseting a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for notebook instances to report information to Notebooks API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportInstanceInfoRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The VM hardware token for authenticating the VM.
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    #[prost(string, tag = "2")]
    pub vm_id: ::prost::alloc::string::String,
    /// The metadata reported to Notebooks API. This will be merged to the instance
    /// metadata store
    #[prost(btree_map = "string, string", tag = "3")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Request for checking if a notebook instance is upgradeable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsInstanceUpgradeableRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub notebook_instance: ::prost::alloc::string::String,
}
/// Response for checking if a notebook instance is upgradeable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsInstanceUpgradeableResponse {
    /// If an instance is upgradeable.
    #[prost(bool, tag = "1")]
    pub upgradeable: bool,
    /// The version this instance will be upgraded to if calling the upgrade
    /// endpoint. This field will only be populated if field upgradeable is true.
    #[prost(string, tag = "2")]
    pub upgrade_version: ::prost::alloc::string::String,
    /// Additional information about upgrade.
    #[prost(string, tag = "3")]
    pub upgrade_info: ::prost::alloc::string::String,
    /// The new image self link this instance will be upgraded to if calling the
    /// upgrade endpoint. This field will only be populated if field upgradeable
    /// is true.
    #[prost(string, tag = "4")]
    pub upgrade_image: ::prost::alloc::string::String,
}
/// Request for checking if a notebook instance is healthy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceHealthRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response for checking if a notebook instance is healthy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceHealthResponse {
    /// Output only. Runtime health_state.
    #[prost(enumeration = "get_instance_health_response::HealthState", tag = "1")]
    pub health_state: i32,
    /// Output only. Additional information about instance health.
    /// Example:
    ///  healthInfo": {
    ///   "docker_proxy_agent_status": "1",
    ///   "docker_status": "1",
    ///   "jupyterlab_api_status": "-1",
    ///   "jupyterlab_status": "-1",
    ///   "updated": "2020-10-18 09:40:03.573409"
    ///  }
    #[prost(btree_map = "string, string", tag = "2")]
    pub health_info: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `GetInstanceHealthResponse`.
pub mod get_instance_health_response {
    /// If an instance is healthy or not.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HealthState {
        /// The instance substate is unknown.
        Unspecified = 0,
        /// The instance is known to be in an healthy state
        /// (for example, critical daemons are running)
        /// Applies to ACTIVE state.
        Healthy = 1,
        /// The instance is known to be in an unhealthy state
        /// (for example, critical daemons are not running)
        /// Applies to ACTIVE state.
        Unhealthy = 2,
        /// The instance has not installed health monitoring agent.
        /// Applies to ACTIVE state.
        AgentNotInstalled = 3,
        /// The instance health monitoring agent is not running.
        /// Applies to ACTIVE state.
        AgentNotRunning = 4,
    }
}
/// Request for upgrading a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for rollbacking a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The snapshot for rollback.
    /// Example: "projects/test-project/global/snapshots/krwlzipynril".
    #[prost(string, tag = "2")]
    pub target_snapshot: ::prost::alloc::string::String,
}
/// Request for upgrading a notebook instance from within the VM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceInternalRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The VM hardware token for authenticating the VM.
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    #[prost(string, tag = "2")]
    pub vm_id: ::prost::alloc::string::String,
}
/// Request for listing environments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// Required. Format: `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing from
    /// the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing environments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// A list of returned environments.
    #[prost(message, repeated, tag = "1")]
    pub environments: ::prost::alloc::vec::Vec<Environment>,
    /// A page token that can be used to continue listing from the last result
    /// in the next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for creating a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentRequest {
    /// Required. Format: `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this environment. The `environment_id` must
    /// be 1 to 63 characters long and contain only lowercase letters,
    /// numeric characters, and dashes. The first character must be a lowercase
    /// letter and the last character cannot be a dash.
    #[prost(string, tag = "2")]
    pub environment_id: ::prost::alloc::string::String,
    /// Required. The environment to be created.
    #[prost(message, optional, tag = "3")]
    pub environment: ::core::option::Option<Environment>,
}
/// Request for deleting a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing scheduled notebook job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchedulesRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing
    /// from the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter applied to resulting schedules.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to order results by.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for listing scheduled notebook job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSchedulesResponse {
    /// A list of returned instances.
    #[prost(message, repeated, tag = "1")]
    pub schedules: ::prost::alloc::vec::Vec<Schedule>,
    /// Page token that can be used to continue listing from the last result in the
    /// next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Schedules that could not be reached. For example:
    ///
    ///     ['projects/{project_id}/location/{location}/schedules/monthly_digest',
    ///      'projects/{project_id}/location/{location}/schedules/weekly_sentiment']
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting scheduled notebook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScheduleRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for deleting an Schedule
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteScheduleRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for created scheduled notebooks
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScheduleRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this schedule.
    #[prost(string, tag = "2")]
    pub schedule_id: ::prost::alloc::string::String,
    /// Required. The schedule to be created.
    #[prost(message, optional, tag = "3")]
    pub schedule: ::core::option::Option<Schedule>,
}
/// Request for created scheduled notebooks
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerScheduleRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}/schedules/{schedule_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing scheduled notebook executions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing
    /// from the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter applied to resulting executions. Currently only supports filtering
    /// executions by a specified schedule_id.
    /// Format: `schedule_id=<Schedule_ID>`
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort by field.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for listing scheduled notebook executions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsResponse {
    /// A list of returned instances.
    #[prost(message, repeated, tag = "1")]
    pub executions: ::prost::alloc::vec::Vec<Execution>,
    /// Page token that can be used to continue listing from the last result in the
    /// next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Executions IDs that could not be reached. For example:
    ///
    ///     ['projects/{project_id}/location/{location}/executions/imagenet_test1',
    ///      'projects/{project_id}/location/{location}/executions/classifier_train1']
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting scheduled notebook execution
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/executions/{execution_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for deleting a scheduled notebook execution
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteExecutionRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/executions/{execution_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to create notebook execution
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExecutionRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this execution.
    #[prost(string, tag = "2")]
    pub execution_id: ::prost::alloc::string::String,
    /// Required. The execution to be created.
    #[prost(message, optional, tag = "3")]
    pub execution: ::core::option::Option<Execution>,
}
#[doc = r" Generated client implementations."]
pub mod notebook_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " API v1 service for Cloud AI Platform Notebooks."]
    #[derive(Debug, Clone)]
    pub struct NotebookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NotebookServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NotebookServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            NotebookServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists instances in a given project and location."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Instance."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Instance in a given project and location."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Registers an existing legacy notebook instance to the Notebooks API server."]
        #[doc = " Legacy instances are instances created with the legacy Compute Engine"]
        #[doc = " calls. They are not manageable by the Notebooks API out of the box. This"]
        #[doc = " call makes these instances manageable by the Notebooks API."]
        pub async fn register_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/RegisterInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the guest accelerators of a single Instance."]
        pub async fn set_instance_accelerator(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInstanceAcceleratorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/SetInstanceAccelerator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the machine type of a single Instance."]
        pub async fn set_instance_machine_type(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInstanceMachineTypeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/SetInstanceMachineType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update Notebook Instance configurations."]
        pub async fn update_instance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/UpdateInstanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the Shielded instance configuration of a single Instance."]
        pub async fn update_shielded_instance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateShieldedInstanceConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/UpdateShieldedInstanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Replaces all the labels of an Instance."]
        pub async fn set_instance_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInstanceLabelsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/SetInstanceLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Instance."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts a notebook instance."]
        pub async fn start_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StartInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/StartInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a notebook instance."]
        pub async fn stop_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/StopInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resets a notebook instance."]
        pub async fn reset_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/ResetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Allows notebook instances to"]
        #[doc = " report their latest instance information to the Notebooks"]
        #[doc = " API server. The server will merge the reported information to"]
        #[doc = " the instance metadata store. Do not use this method directly."]
        pub async fn report_instance_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportInstanceInfoRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/ReportInstanceInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Check if a notebook instance is upgradable."]
        pub async fn is_instance_upgradeable(
            &mut self,
            request: impl tonic::IntoRequest<super::IsInstanceUpgradeableRequest>,
        ) -> Result<tonic::Response<super::IsInstanceUpgradeableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/IsInstanceUpgradeable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Check if a notebook instance is healthy."]
        pub async fn get_instance_health(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceHealthRequest>,
        ) -> Result<tonic::Response<super::GetInstanceHealthResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/GetInstanceHealth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Upgrades a notebook instance to the latest version."]
        pub async fn upgrade_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/UpgradeInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Rollbacks a notebook instance to the previous version."]
        pub async fn rollback_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/RollbackInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Allows notebook instances to"]
        #[doc = " call this endpoint to upgrade themselves. Do not use this method directly."]
        pub async fn upgrade_instance_internal(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeInstanceInternalRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/UpgradeInstanceInternal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists environments in a project."]
        pub async fn list_environments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnvironmentsRequest>,
        ) -> Result<tonic::Response<super::ListEnvironmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/ListEnvironments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Environment."]
        pub async fn get_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentRequest>,
        ) -> Result<tonic::Response<super::Environment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/GetEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Environment."]
        pub async fn create_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/CreateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Environment."]
        pub async fn delete_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/DeleteEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists schedules in a given project and location."]
        pub async fn list_schedules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSchedulesRequest>,
        ) -> Result<tonic::Response<super::ListSchedulesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/ListSchedules",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of schedule"]
        pub async fn get_schedule(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScheduleRequest>,
        ) -> Result<tonic::Response<super::Schedule>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/GetSchedule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes schedule and all underlying jobs"]
        pub async fn delete_schedule(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteScheduleRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/DeleteSchedule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Scheduled Notebook in a given project and location."]
        pub async fn create_schedule(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateScheduleRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/CreateSchedule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Triggers execution of an existing schedule."]
        pub async fn trigger_schedule(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerScheduleRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/TriggerSchedule",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists executions in a given project and location"]
        pub async fn list_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionsRequest>,
        ) -> Result<tonic::Response<super::ListExecutionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/ListExecutions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of executions"]
        pub async fn get_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionRequest>,
        ) -> Result<tonic::Response<super::Execution>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/GetExecution",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes execution"]
        pub async fn delete_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteExecutionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/DeleteExecution",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Scheduled Notebook in a given project and location."]
        pub async fn create_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExecutionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.NotebookService/CreateExecution",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The definition of a Runtime for a managed notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Runtime {
    /// Output only. The resource name of the runtime.
    /// Format:
    /// `projects/{project}/locations/{location}/runtimes/{runtimeId}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Runtime state.
    #[prost(enumeration = "runtime::State", tag = "3")]
    pub state: i32,
    /// Output only. Runtime health_state.
    #[prost(enumeration = "runtime::HealthState", tag = "4")]
    pub health_state: i32,
    /// The config settings for accessing runtime.
    #[prost(message, optional, tag = "5")]
    pub access_config: ::core::option::Option<RuntimeAccessConfig>,
    /// The config settings for software inside the runtime.
    #[prost(message, optional, tag = "6")]
    pub software_config: ::core::option::Option<RuntimeSoftwareConfig>,
    /// Output only. Contains Runtime daemon metrics such as Service status and JupyterLab
    /// stats.
    #[prost(message, optional, tag = "7")]
    pub metrics: ::core::option::Option<RuntimeMetrics>,
    /// Output only. Runtime creation time.
    #[prost(message, optional, tag = "20")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Runtime update time.
    #[prost(message, optional, tag = "21")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the runtime; currently only supports Compute Engine VM.
    #[prost(oneof = "runtime::RuntimeType", tags = "2")]
    pub runtime_type: ::core::option::Option<runtime::RuntimeType>,
}
/// Nested message and enum types in `Runtime`.
pub mod runtime {
    /// The definition of the states of this runtime.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State is not specified.
        Unspecified = 0,
        /// The compute layer is starting the runtime. It is not ready for use.
        Starting = 1,
        /// The compute layer is installing required frameworks and registering the
        /// runtime with notebook proxy. It cannot be used.
        Provisioning = 2,
        /// The runtime is currently running. It is ready for use.
        Active = 3,
        /// The control logic is stopping the runtime. It cannot be used.
        Stopping = 4,
        /// The runtime is stopped. It cannot be used.
        Stopped = 5,
        /// The runtime is being deleted. It cannot be used.
        Deleting = 6,
        /// The runtime is upgrading. It cannot be used.
        Upgrading = 7,
        /// The runtime is being created and set up. It is not ready for use.
        Initializing = 8,
    }
    /// The runtime substate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HealthState {
        /// The runtime substate is unknown.
        Unspecified = 0,
        /// The runtime is known to be in an healthy state
        /// (for example, critical daemons are running)
        /// Applies to ACTIVE state.
        Healthy = 1,
        /// The runtime is known to be in an unhealthy state
        /// (for example, critical daemons are not running)
        /// Applies to ACTIVE state.
        Unhealthy = 2,
    }
    /// Type of the runtime; currently only supports Compute Engine VM.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuntimeType {
        /// Use a Compute Engine VM image to start the managed notebook instance.
        #[prost(message, tag = "2")]
        VirtualMachine(super::VirtualMachine),
    }
}
/// Definition of the types of hardware accelerators that can be used.
/// Definition of the types of hardware accelerators that can be used.
/// See [Compute Engine
/// AcceleratorTypes](<https://cloud.google.com/compute/docs/reference/beta/acceleratorTypes>).
/// Examples:
///
/// * `nvidia-tesla-k80`
/// * `nvidia-tesla-p100`
/// * `nvidia-tesla-v100`
/// * `nvidia-tesla-t4`
/// * `nvidia-tesla-a100`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeAcceleratorConfig {
    /// Accelerator model.
    #[prost(enumeration = "runtime_accelerator_config::AcceleratorType", tag = "1")]
    pub r#type: i32,
    /// Count of cores of this accelerator.
    #[prost(int64, tag = "2")]
    pub core_count: i64,
}
/// Nested message and enum types in `RuntimeAcceleratorConfig`.
pub mod runtime_accelerator_config {
    /// Type of this accelerator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AcceleratorType {
        /// Accelerator type is not specified.
        Unspecified = 0,
        /// Accelerator type is Nvidia Tesla K80.
        NvidiaTeslaK80 = 1,
        /// Accelerator type is Nvidia Tesla P100.
        NvidiaTeslaP100 = 2,
        /// Accelerator type is Nvidia Tesla V100.
        NvidiaTeslaV100 = 3,
        /// Accelerator type is Nvidia Tesla P4 GPU.
        NvidiaTeslaP4 = 4,
        /// Accelerator type is Nvidia Tesla T4.
        NvidiaTeslaT4 = 5,
        /// Accelerator type is Nvidia Tesla A100.
        NvidiaTeslaA100 = 6,
        /// (Coming soon) Accelerator type is TPU V2.
        TpuV2 = 7,
        /// (Coming soon) Accelerator type is TPU V3.
        TpuV3 = 8,
        /// Accelerator type is NVIDIA Tesla T4 Virtual Workstations.
        NvidiaTeslaT4Vws = 9,
        /// Accelerator type is NVIDIA Tesla P100 Virtual Workstations.
        NvidiaTeslaP100Vws = 10,
        /// Accelerator type is Nvidia Tesla P.4 GPU Virtual Workstations.
        NvidiaTeslaP4Vws = 11,
    }
}
/// Represents a custom encryption key configuration that can be applied to
/// a resource. This will encrypt all disks in Virtual Machine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionConfig {
    /// The Cloud KMS resource identifier of the customer-managed encryption key
    /// used to protect a resource, such as a disks. It has the following
    /// format:
    /// `projects/{PROJECT_ID}/locations/{REGION}/keyRings/{KEY_RING_NAME}/cryptoKeys/{KEY_NAME}`
    #[prost(string, tag = "1")]
    pub kms_key: ::prost::alloc::string::String,
}
/// A Local attached disk resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalDisk {
    /// Optional. Output only. Specifies whether the disk will be auto-deleted when the
    /// instance is deleted (but not when the disk is detached from the instance).
    #[prost(bool, tag = "1")]
    pub auto_delete: bool,
    /// Optional. Output only. Indicates that this is a boot disk. The virtual machine
    /// will use the first partition of the disk for its root filesystem.
    #[prost(bool, tag = "2")]
    pub boot: bool,
    /// Optional. Output only. Specifies a unique device name
    /// of your choice that is reflected into the
    /// /dev/disk/by-id/google-* tree of a Linux operating system running within
    /// the instance. This name can be used to reference the device for mounting,
    /// resizing, and so on, from within the instance.
    ///
    /// If not specified, the server chooses a default device name to apply to this
    /// disk, in the form persistent-disk-x, where x is a number assigned by Google
    /// Compute Engine. This field is only applicable for persistent disks.
    #[prost(string, tag = "3")]
    pub device_name: ::prost::alloc::string::String,
    /// Output only. Indicates a list of features to enable on the guest operating system.
    /// Applicable only for bootable images. Read  Enabling guest operating
    /// system features to see a list of available options.
    #[prost(message, repeated, tag = "4")]
    pub guest_os_features: ::prost::alloc::vec::Vec<local_disk::RuntimeGuestOsFeature>,
    /// Output only. A zero-based index to this disk, where 0 is reserved for the
    /// boot disk. If you have many disks attached to an instance, each disk would
    /// have a unique index number.
    #[prost(int32, tag = "5")]
    pub index: i32,
    /// Input only. Specifies the parameters for a new disk that will be created
    /// alongside the new instance. Use initialization parameters to create boot
    /// disks or local SSDs attached to the new instance.
    ///
    /// This property is mutually exclusive with the source property; you can only
    /// define one or the other, but not both.
    #[prost(message, optional, tag = "6")]
    pub initialize_params: ::core::option::Option<LocalDiskInitializeParams>,
    /// Specifies the disk interface to use for attaching this disk, which is
    /// either SCSI or NVME. The default is SCSI. Persistent disks must always use
    /// SCSI and the request will fail if you attempt to attach a persistent disk
    /// in any other format than SCSI. Local SSDs can use either NVME or SCSI. For
    /// performance characteristics of SCSI over NVMe, see Local SSD performance.
    /// Valid values:
    ///
    /// * NVME
    /// * SCSI
    #[prost(string, tag = "7")]
    pub interface: ::prost::alloc::string::String,
    /// Output only. Type of the resource. Always compute#attachedDisk for attached disks.
    #[prost(string, tag = "8")]
    pub kind: ::prost::alloc::string::String,
    /// Output only. Any valid publicly visible licenses.
    #[prost(string, repeated, tag = "9")]
    pub licenses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The mode in which to attach this disk, either READ_WRITE or READ_ONLY. If
    /// not specified, the default is to attach the disk in READ_WRITE mode.
    /// Valid values:
    ///
    /// * READ_ONLY
    /// * READ_WRITE
    #[prost(string, tag = "10")]
    pub mode: ::prost::alloc::string::String,
    /// Specifies a valid partial or full URL to an existing Persistent Disk
    /// resource.
    #[prost(string, tag = "11")]
    pub source: ::prost::alloc::string::String,
    /// Specifies the type of the disk, either SCRATCH or PERSISTENT. If not
    /// specified, the default is PERSISTENT.
    /// Valid values:
    ///
    /// * PERSISTENT
    /// * SCRATCH
    #[prost(string, tag = "12")]
    pub r#type: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LocalDisk`.
pub mod local_disk {
    /// Optional. A list of features to enable on the guest operating system.
    /// Applicable only for bootable images.
    /// Read [Enabling guest operating system
    /// features](<https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features>)
    /// to see a list of available options.
    /// Guest OS features for boot disk.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RuntimeGuestOsFeature {
        /// The ID of a supported feature. Read [Enabling guest operating system
        /// features](<https://cloud.google.com/compute/docs/images/create-delete-deprecate-private-images#guest-os-features>)
        /// to see a list of available options.
        ///
        /// Valid values:
        ///
        /// * FEATURE_TYPE_UNSPECIFIED
        /// * MULTI_IP_SUBNET
        /// * SECURE_BOOT
        /// * UEFI_COMPATIBLE
        /// * VIRTIO_SCSI_MULTIQUEUE
        /// * WINDOWS
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
    }
}
/// Input only. Specifies the parameters for a new disk that will be created
/// alongside the new instance. Use initialization parameters to create boot
/// disks or local SSDs attached to the new runtime.
/// This property is mutually exclusive with the source property; you can only
/// define one or the other, but not both.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalDiskInitializeParams {
    /// Optional. Provide this property when creating the disk.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Specifies the disk name. If not specified, the default is to use the name
    /// of the instance. If the disk with the instance name exists already in the
    /// given zone/region, a new name will be automatically generated.
    #[prost(string, tag = "2")]
    pub disk_name: ::prost::alloc::string::String,
    /// Optional. Specifies the size of the disk in base-2 GB. If not specified, the disk
    /// will be the same size as the image (usually 10GB). If specified, the size
    /// must be equal to or larger than 10GB. Default 100 GB.
    #[prost(int64, tag = "3")]
    pub disk_size_gb: i64,
    /// Input only. The type of the boot disk attached to this instance, defaults to
    /// standard persistent disk (`PD_STANDARD`).
    #[prost(enumeration = "local_disk_initialize_params::DiskType", tag = "4")]
    pub disk_type: i32,
    /// Optional. Labels to apply to this disk. These can be later modified by the
    /// disks.setLabels method. This field is only applicable for persistent disks.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `LocalDiskInitializeParams`.
pub mod local_disk_initialize_params {
    /// Possible disk types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiskType {
        /// Disk type not set.
        Unspecified = 0,
        /// Standard persistent disk type.
        PdStandard = 1,
        /// SSD persistent disk type.
        PdSsd = 2,
        /// Balanced persistent disk type.
        PdBalanced = 3,
    }
}
/// Specifies the login configuration for Runtime
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeAccessConfig {
    /// The type of access mode this instance.
    #[prost(enumeration = "runtime_access_config::RuntimeAccessType", tag = "1")]
    pub access_type: i32,
    /// The owner of this runtime after creation. Format: `alias@example.com`
    /// Currently supports one owner only.
    #[prost(string, tag = "2")]
    pub runtime_owner: ::prost::alloc::string::String,
    /// Output only. The proxy endpoint that is used to access the runtime.
    #[prost(string, tag = "3")]
    pub proxy_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RuntimeAccessConfig`.
pub mod runtime_access_config {
    /// Possible ways to access runtime. Authentication mode.
    /// Currently supports: Single User only.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RuntimeAccessType {
        /// Unspecified access.
        Unspecified = 0,
        /// Single user login.
        SingleUser = 1,
    }
}
/// Specifies the selection and configuration of software inside the runtime.
/// The properties to set on runtime.
/// Properties keys are specified in `key:value` format, for example:
///
/// * `idle_shutdown: true`
/// * `idle_shutdown_timeout: 180`
/// * `report-system-health: true`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeSoftwareConfig {
    /// Cron expression in UTC timezone, used to schedule instance auto upgrade.
    /// Please follow the [cron format](<https://en.wikipedia.org/wiki/Cron>).
    #[prost(string, tag = "1")]
    pub notebook_upgrade_schedule: ::prost::alloc::string::String,
    /// Verifies core internal services are running.
    /// Default: True
    #[prost(bool, optional, tag = "2")]
    pub enable_health_monitoring: ::core::option::Option<bool>,
    /// Runtime will automatically shutdown after idle_shutdown_time.
    /// Default: True
    #[prost(bool, optional, tag = "3")]
    pub idle_shutdown: ::core::option::Option<bool>,
    /// Time in minutes to wait before shutting down runtime. Default: 180 minutes
    #[prost(int32, tag = "4")]
    pub idle_shutdown_timeout: i32,
    /// Install Nvidia Driver automatically.
    #[prost(bool, tag = "5")]
    pub install_gpu_driver: bool,
    /// Specify a custom Cloud Storage path where the GPU driver is stored.
    /// If not specified, we'll automatically choose from official GPU drivers.
    #[prost(string, tag = "6")]
    pub custom_gpu_driver_path: ::prost::alloc::string::String,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path (`gs://path-to-file/file-name`).
    #[prost(string, tag = "7")]
    pub post_startup_script: ::prost::alloc::string::String,
}
/// Contains runtime daemon metrics, such as OS and kernels and sessions stats.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeMetrics {
    /// Output only. The system metrics.
    #[prost(btree_map = "string, string", tag = "1")]
    pub system_metrics: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// A set of Shielded Instance options.
/// Check [Images using supported Shielded VM
/// features](<https://cloud.google.com/compute/docs/instances/modifying-shielded-vm>).
/// Not all combinations are valid.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeShieldedInstanceConfig {
    /// Defines whether the instance has Secure Boot enabled.
    ///
    /// Secure Boot helps ensure that the system only runs authentic software by
    /// verifying the digital signature of all boot components, and halting the
    /// boot process if signature verification fails. Disabled by default.
    #[prost(bool, tag = "1")]
    pub enable_secure_boot: bool,
    /// Defines whether the instance has the vTPM enabled. Enabled by default.
    #[prost(bool, tag = "2")]
    pub enable_vtpm: bool,
    /// Defines whether the instance has integrity monitoring enabled.
    ///
    /// Enables monitoring and attestation of the boot integrity of the instance.
    /// The attestation is performed against the integrity policy baseline. This
    /// baseline is initially derived from the implicitly trusted boot image when
    /// the instance is created. Enabled by default.
    #[prost(bool, tag = "3")]
    pub enable_integrity_monitoring: bool,
}
/// Runtime using Virtual Machine for computing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachine {
    /// Output only. The user-friendly name of the Managed Compute Engine instance.
    #[prost(string, tag = "1")]
    pub instance_name: ::prost::alloc::string::String,
    /// Output only. The unique identifier of the Managed Compute Engine instance.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Virtual Machine configuration settings.
    #[prost(message, optional, tag = "3")]
    pub virtual_machine_config: ::core::option::Option<VirtualMachineConfig>,
}
/// The config settings for virtual machine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualMachineConfig {
    /// Output only. The zone where the virtual machine is located.
    /// If using regional request, the notebooks service will pick a location
    /// in the corresponding runtime region.
    /// On a get request, zone will always be present. Example:
    /// * `us-central1-b`
    #[prost(string, tag = "1")]
    pub zone: ::prost::alloc::string::String,
    /// Required. The Compute Engine machine type used for runtimes.
    /// Short name is valid. Examples:
    /// * `n1-standard-2`
    /// * `e2-standard-8`
    #[prost(string, tag = "2")]
    pub machine_type: ::prost::alloc::string::String,
    /// Optional. Use a list of container images to start the notebook instance.
    #[prost(message, repeated, tag = "3")]
    pub container_images: ::prost::alloc::vec::Vec<ContainerImage>,
    /// Required. Data disk option configuration settings.
    #[prost(message, optional, tag = "4")]
    pub data_disk: ::core::option::Option<LocalDisk>,
    /// Optional. Encryption settings for virtual machine data disk.
    #[prost(message, optional, tag = "5")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
    /// Optional. Shielded VM Instance configuration settings.
    #[prost(message, optional, tag = "6")]
    pub shielded_instance_config: ::core::option::Option<RuntimeShieldedInstanceConfig>,
    /// Optional. The Compute Engine accelerator configuration for this runtime.
    #[prost(message, optional, tag = "7")]
    pub accelerator_config: ::core::option::Option<RuntimeAcceleratorConfig>,
    /// Optional. The Compute Engine network to be used for machine
    /// communications. Cannot be specified with subnetwork. If neither
    /// `network` nor `subnet` is specified, the "default" network of
    /// the project is used, if it exists.
    ///
    /// A full URL or partial URI. Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id\]/regions/global/default`>
    /// * `projects/\[project_id\]/regions/global/default`
    ///
    /// Runtimes are managed resources inside Google Infrastructure.
    /// Runtimes support the following network configurations:
    ///
    /// * Google Managed Network (Network & subnet are empty)
    /// * Consumer Project VPC (network & subnet are required). Requires
    /// configuring Private Service Access.
    /// * Shared VPC (network & subnet are required). Requires configuring Private
    /// Service Access.
    #[prost(string, tag = "8")]
    pub network: ::prost::alloc::string::String,
    /// Optional. The Compute Engine subnetwork to be used for machine
    /// communications. Cannot be specified with network.
    ///
    /// A full URL or partial URI are valid. Examples:
    ///
    /// * `<https://www.googleapis.com/compute/v1/projects/\[project_id\]/regions/us-east1/subnetworks/sub0`>
    /// * `projects/\[project_id\]/regions/us-east1/subnetworks/sub0`
    #[prost(string, tag = "9")]
    pub subnet: ::prost::alloc::string::String,
    /// Optional. If true, runtime will only have internal IP
    /// addresses. By default, runtimes are not restricted to internal IP
    /// addresses, and will have ephemeral external IP addresses assigned to each
    /// vm. This `internal_ip_only` restriction can only be enabled for
    /// subnetwork enabled networks, and all dependencies must be
    /// configured to be accessible without external IP addresses.
    #[prost(bool, tag = "10")]
    pub internal_ip_only: bool,
    /// Optional. The Compute Engine tags to add to runtime (see [Tagging
    /// instances](<https://cloud.google.com/compute/docs/label-or-tag-resources#tags>)).
    #[prost(string, repeated, tag = "13")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The Compute Engine guest attributes. (see
    /// [Project and instance
    /// guest
    /// attributes](<https://cloud.google.com/compute/docs/storing-retrieving-metadata#guest_attributes>)).
    #[prost(btree_map = "string, string", tag = "14")]
    pub guest_attributes: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The Compute Engine metadata entries to add to virtual machine. (see
    /// [Project and instance
    /// metadata](<https://cloud.google.com/compute/docs/storing-retrieving-metadata#project_and_instance_metadata>)).
    #[prost(btree_map = "string, string", tag = "15")]
    pub metadata: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The labels to associate with this runtime.
    /// Label **keys** must contain 1 to 63 characters, and must conform to
    /// [RFC 1035](<https://www.ietf.org/rfc/rfc1035.txt>).
    /// Label **values** may be empty, but, if present, must contain 1 to 63
    /// characters, and must conform to [RFC
    /// 1035](<https://www.ietf.org/rfc/rfc1035.txt>). No more than 32 labels can be
    /// associated with a cluster.
    #[prost(btree_map = "string, string", tag = "16")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The type of vNIC to be used on this interface. This may be gVNIC or
    /// VirtioNet.
    #[prost(enumeration = "virtual_machine_config::NicType", tag = "17")]
    pub nic_type: i32,
}
/// Nested message and enum types in `VirtualMachineConfig`.
pub mod virtual_machine_config {
    /// The type of vNIC driver.
    /// Default should be UNSPECIFIED_NIC_TYPE.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NicType {
        /// No type specified.
        UnspecifiedNicType = 0,
        /// VIRTIO
        VirtioNet = 1,
        /// GVNIC
        Gvnic = 2,
    }
}
/// The definition of an Event for a managed / semi-managed notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Event report time.
    #[prost(message, optional, tag = "1")]
    pub report_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Event type.
    #[prost(enumeration = "event::EventType", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// The definition of the even types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// Event is not specified.
        Unspecified = 0,
        /// The instance / runtime is idle
        Idle = 1,
    }
}
/// Request for listing Managed Notebook Runtimes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimesRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing
    /// from the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing Managed Notebook Runtimes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimesResponse {
    /// A list of returned Runtimes.
    #[prost(message, repeated, tag = "1")]
    pub runtimes: ::prost::alloc::vec::Vec<Runtime>,
    /// Page token that can be used to continue listing from the last result in the
    /// next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. For example,
    /// ['us-west1', 'us-central1'].
    /// A ListRuntimesResponse will only contain either runtimes or unreachables,
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting a Managed Notebook Runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRuntimeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for creating a Managed Notebook Runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRuntimeRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this Runtime.
    #[prost(string, tag = "2")]
    pub runtime_id: ::prost::alloc::string::String,
    /// Required. The Runtime to be created.
    #[prost(message, optional, tag = "3")]
    pub runtime: ::core::option::Option<Runtime>,
}
/// Request for deleting a Managed Notebook Runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRuntimeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for starting a Managed Notebook Runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRuntimeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for stopping a Managed Notebook Runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRuntimeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for switching a Managed Notebook Runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchRuntimeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// machine type.
    #[prost(string, tag = "2")]
    pub machine_type: ::prost::alloc::string::String,
    /// accelerator config.
    #[prost(message, optional, tag = "3")]
    pub accelerator_config: ::core::option::Option<RuntimeAcceleratorConfig>,
}
/// Request for reseting a Managed Notebook Runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRuntimeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for reporting a Managed Notebook Event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRuntimeEventRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/runtimes/{runtime_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The VM hardware token for authenticating the VM.
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    #[prost(string, tag = "2")]
    pub vm_id: ::prost::alloc::string::String,
    /// Required. The Event to be reported.
    #[prost(message, optional, tag = "3")]
    pub event: ::core::option::Option<Event>,
}
#[doc = r" Generated client implementations."]
pub mod managed_notebook_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " API v1 service for Managed Notebooks."]
    #[derive(Debug, Clone)]
    pub struct ManagedNotebookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ManagedNotebookServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ManagedNotebookServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ManagedNotebookServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists Runtimes in a given project and location."]
        pub async fn list_runtimes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuntimesRequest>,
        ) -> Result<tonic::Response<super::ListRuntimesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/ListRuntimes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Runtime. The location must be a regional endpoint"]
        #[doc = " rather than zonal."]
        pub async fn get_runtime(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRuntimeRequest>,
        ) -> Result<tonic::Response<super::Runtime>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/GetRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Runtime in a given project and location."]
        pub async fn create_runtime(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRuntimeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/CreateRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Runtime."]
        pub async fn delete_runtime(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRuntimeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/DeleteRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts a Managed Notebook Runtime."]
        #[doc = " Perform \"Start\" on GPU instances; \"Resume\" on CPU instances"]
        #[doc = " See:"]
        #[doc = " https://cloud.google.com/compute/docs/instances/stop-start-instance"]
        #[doc = " https://cloud.google.com/compute/docs/instances/suspend-resume-instance"]
        pub async fn start_runtime(
            &mut self,
            request: impl tonic::IntoRequest<super::StartRuntimeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/StartRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a Managed Notebook Runtime."]
        #[doc = " Perform \"Stop\" on GPU instances; \"Suspend\" on CPU instances"]
        #[doc = " See:"]
        #[doc = " https://cloud.google.com/compute/docs/instances/stop-start-instance"]
        #[doc = " https://cloud.google.com/compute/docs/instances/suspend-resume-instance"]
        pub async fn stop_runtime(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRuntimeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/StopRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Switch a Managed Notebook Runtime."]
        pub async fn switch_runtime(
            &mut self,
            request: impl tonic::IntoRequest<super::SwitchRuntimeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/SwitchRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resets a Managed Notebook Runtime."]
        pub async fn reset_runtime(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetRuntimeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/ResetRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Report and process a runtime event."]
        pub async fn report_runtime_event(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportRuntimeEventRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1.ManagedNotebookService/ReportRuntimeEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
