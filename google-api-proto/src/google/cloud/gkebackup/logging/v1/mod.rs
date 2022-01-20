/// Restore as stored in Platform log. It's used to log the update details of a
/// updateRestore request, so only mutable and non-output_only fields are
/// included here..
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggedRestore {
    /// Full name of the Backup resource this Restore resource used to restore
    /// from. Format: projects/*/locations/*/backupPlans/*/backups/*.
    #[prost(string, tag = "1")]
    pub backup: ::prost::alloc::string::String,
    /// GCP Labels.
    #[prost(btree_map = "string, string", tag = "2")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// User specified descriptive string for this Restore.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The current state of the Restore.
    #[prost(enumeration = "logged_restore::State", tag = "4")]
    pub state: i32,
    /// Human-readable description of why the Restore is in its current state.
    #[prost(string, tag = "5")]
    pub state_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LoggedRestore`.
pub mod logged_restore {
    /// Possible values for state of the Restore.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The Restore resource is in the process of being created.
        Unspecified = 0,
        /// The Restore resource has been created and the associated RestoreJob
        /// Kubernetes resource has been injected into target cluster.
        Creating = 1,
        /// The gkebackup agent in the cluster has begun executing the restore
        /// operation.
        InProgress = 2,
        /// The restore operation has completed successfully. Restored workloads may
        /// not yet be operational.
        Succeeded = 3,
        /// The restore operation has failed.
        Failed = 4,
        /// This Restore resource is in the process of being deleted.
        Deleting = 5,
    }
}
/// Namespaces, list of namespaces
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespaces {
    /// namespaces
    #[prost(string, repeated, tag = "1")]
    pub namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// NamespacedName
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespacedName {
    /// the namespace of the resource in Kubernetes
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// the name of the resource in Kubernetes
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// NamespacedNames
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespacedNames {
    /// a list of namespaced names in Kubernetes
    #[prost(message, repeated, tag = "1")]
    pub namespaced_names: ::prost::alloc::vec::Vec<NamespacedName>,
}
/// Encryption key.
/// This only contains the key metadata, and no key material.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionKey {
    /// Google KMS encryption key in the format:
    /// projects/<project>/locations/<location>/keyRings/<key-ring>/cryptoKeys/<key>
    #[prost(string, tag = "1")]
    pub gcp_kms_encryption_key: ::prost::alloc::string::String,
}
/// RestorePlan as stored in Platform log. It's used to log the details of
/// a createRestorePlan/updateRestorePlan request, so only fields that can be
/// taken from user input are included here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggedRestorePlan {
    /// User specified descriptive string for this RestorePlan.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// The BackupPlan from which Backups may be used as the source
    /// for Restores created via this RestorePlan.
    /// Format: projects/*/locations/*/backupPlans/*.
    #[prost(string, tag = "2")]
    pub backup_plan: ::prost::alloc::string::String,
    /// The target cluster into which Restores created via this RestorePlan
    /// will restore data. NOTE: the cluster's region must be the same as the
    /// RestorePlan.
    /// Possible formats:
    ///   1. projects/*/locations/*/clusters/*
    ///   2. projects/*/zones/*/clusters/*
    #[prost(string, tag = "3")]
    pub cluster: ::prost::alloc::string::String,
    /// Configuration of Restores created via this RestorePlan.
    #[prost(message, optional, tag = "4")]
    pub restore_config: ::core::option::Option<RestoreConfig>,
    /// A set of custom labels supplied by user.
    #[prost(btree_map = "string, string", tag = "5")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Configuration of a restore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreConfig {
    /// Specifies the mechanism to be used to restore volume data.
    /// Default: VOLUME_DATA_RESTORE_POLICY_UNSPECIFIED (will be treated as
    /// NO_VOLUME_DATA_RESTORATION).
    #[prost(enumeration = "restore_config::VolumeDataRestorePolicy", tag = "1")]
    pub volume_data_restore_policy: i32,
    /// Defines the behavior for handling the situation where cluster-scoped
    /// resources being restored already exist in the target cluster. This MUST be
    /// set to a value other than CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED if
    /// cluster_resource_restore_scope is not empty.
    #[prost(
        enumeration = "restore_config::ClusterResourceConflictPolicy",
        tag = "2"
    )]
    pub cluster_resource_conflict_policy: i32,
    /// Defines the behavior for handling the situation where sets of namespaced
    /// resources being restored already exist in the target cluster. This MUST be
    /// set to a value other than NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED if
    /// any namespaced restoration is configured via
    /// namespaced_resource_restore_scope .
    #[prost(
        enumeration = "restore_config::NamespacedResourceRestoreMode",
        tag = "3"
    )]
    pub namespaced_resource_restore_mode: i32,
    /// Identifies the cluster-scoped resources to restore from the Backup.
    /// Not specifying it means NO cluster resource will be restored.
    #[prost(message, optional, tag = "4")]
    pub cluster_resource_restore_scope:
        ::core::option::Option<restore_config::ClusterResourceRestoreScope>,
    /// A list of transformation rules to be applied against Kubernetes resources
    /// as they are selected for restoration from a Backup. Rules are executed in
    /// order defined - this order matters, as changes made by a rule may impact
    /// the filtering logic of subsequent rules. An empty list means no
    /// substitution will occur.
    #[prost(message, repeated, tag = "8")]
    pub substitution_rules: ::prost::alloc::vec::Vec<restore_config::SubstitutionRule>,
    /// Specifies the namespaced resources to restore from the Backup.
    /// Only one of the entries may be specified. If not specified, NO namespaced
    /// resources will be restored.
    #[prost(
        oneof = "restore_config::NamespacedResourceRestoreScope",
        tags = "5, 6, 7"
    )]
    pub namespaced_resource_restore_scope:
        ::core::option::Option<restore_config::NamespacedResourceRestoreScope>,
}
/// Nested message and enum types in `RestoreConfig`.
pub mod restore_config {
    /// This is a direct map to the Kubernetes GroupKind type
    /// \[GroupKind\](<https://godoc.org/k8s.io/apimachinery/pkg/runtime/schema#GroupKind>)
    /// and is used for identifying specific "types" of resources to restore.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupKind {
        /// API group string of a Kubernetes resource, e.g.
        /// "apiextensions.k8s.io", "storage.k8s.io", etc.
        /// Note: use empty string for core API group
        #[prost(string, tag = "1")]
        pub resource_group: ::prost::alloc::string::String,
        /// Kind of a Kubernetes resource, e.g.
        /// "CustomResourceDefinition", "StorageClass", etc.
        #[prost(string, tag = "2")]
        pub resource_kind: ::prost::alloc::string::String,
    }
    /// Identifies the cluster-scoped resources to restore from the Backup.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterResourceRestoreScope {
        /// A list of "types" of cluster-scoped resources to be restored from the
        /// Backup.  An empty list means that NO cluster-scoped resources will be
        /// restored. Note that Namespaces and PersistentVolume restoration is
        /// handled separately and is not governed by this field.
        #[prost(message, repeated, tag = "1")]
        pub selected_group_kinds: ::prost::alloc::vec::Vec<GroupKind>,
    }
    /// A transformation rule to be applied against Kubernetes resources as they
    /// are selected for restoration from a Backup. A rule contains both filtering
    /// logic (which resources are subject to substitution) and substitution logic.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubstitutionRule {
        /// (Filtering parameter) Any resource subject to substitution must be
        /// contained within one of the listed Kubernetes Namespace in the Backup.
        /// If this field is not provided, no namespace filtering will be performed
        /// (all resources in all Namespaces, including all cluster-scoped resources,
        /// will be candidates for substitution).
        /// To mix cluster-scoped and namespaced resources in the same rule, use an
        /// empty string ("") as one of the target namespaces.
        #[prost(string, repeated, tag = "1")]
        pub target_namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// (Filtering parameter) Any resource subject to substitution must belong to
        /// one of the listed "types".
        /// If this field is not provided, no type filtering will be performed (all
        /// resources of all types matching previous filtering parameters will be
        /// candidates for substitution).
        #[prost(message, repeated, tag = "2")]
        pub target_group_kinds: ::prost::alloc::vec::Vec<GroupKind>,
        /// This is a \[JSONPath\]
        /// ([<https://kubernetes.io/docs/reference/kubectl/jsonpath/>)
        /// expression that matches specific fields of candidate
        /// resources and it operates as both a filtering parameter (resources that
        /// are not matched with this expression will not be candidates for
        /// substitution) as well as a field identifier (identifies exactly which
        /// fields out of the candidate resources will be modified).
        #[prost(string, tag = "3")]
        pub target_json_path: ::prost::alloc::string::String,
        /// (Filtering parameter) This is a [regular expression]
        /// (<https://en.wikipedia.org/wiki/Regular_expression>)
        /// that is compared against the fields matched by the target_json_path
        /// expression (and must also have passed the previous filters).
        /// Substitution will not be performed against fields whose
        /// value does not match this expression. If this field is NOT specified,
        /// then ALL fields matched by the target_json_path expression will undergo
        /// substitution. Note that an empty (e.g., "", rather than unspecified)
        /// value for for this field will only match empty fields.
        #[prost(string, tag = "4")]
        pub original_value_pattern: ::prost::alloc::string::String,
        /// This is the new value to set for any fields that pass the filtering and
        /// selection criteria. To remove a value from a Kubernetes resource, either
        /// leave this field unspecified, or set it to the empty string ("").
        #[prost(string, tag = "5")]
        pub new_value: ::prost::alloc::string::String,
    }
    /// Defines how volume data should be restored
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VolumeDataRestorePolicy {
        /// unspecified, default value
        Unspecified = 0,
        /// For each PVC to be restored, will create a new underlying volume (and PV)
        /// from the corresponding VolumeBackup contained within the Backup.
        RestoreVolumeDataFromBackup = 1,
        /// For each PVC to be restored, attempt to reuse the original PV contained
        /// in the Backup (with its original underlying volume).  Note that option
        /// is likely only usable when restoring a workload to its original cluster.
        ReuseVolumeHandleFromBackup = 2,
        /// For each PVC to be restored, PVCs will be created without any particular
        /// action to restore data.  In this case, the normal Kubernetes provisioning
        /// logic would kick in, and this would likely result in either dynamically
        /// provisioning blank PVs or binding to statically provisioned PVs.
        NoVolumeDataRestoration = 3,
    }
    /// Defines the behavior for handling the situation where cluster-scoped
    /// resources being restored already exist in the target cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClusterResourceConflictPolicy {
        /// Unspecified. Only allowed if no cluster-scoped resources will be
        /// restored.
        Unspecified = 0,
        /// Do not attempt to restore the conflicting resource.
        UseExistingVersion = 1,
        /// Delete the existing version before re-creating it from the Backup.
        /// Note that this is a dangerous option which could cause unintentional
        /// data loss if used inappropriately - for example, deleting a CRD will
        /// cause Kubernetes to delete all CRs of that type.
        UseBackupVersion = 2,
    }
    /// Defines the behavior for handling the situation where sets of namespaced
    /// resources being restored already exist in the target cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NamespacedResourceRestoreMode {
        /// Unspecified. Only allowed if no namespaced resources will be restored.
        Unspecified = 0,
        /// When conflicting top-level resources (either Namespaces or
        /// ProtectedApplications, depending upon the scope) are encountered, this
        /// will first trigger a delete of the conflicting resource AND ALL OF ITS
        /// REFERENCED RESOURCES (e.g., all resources in the Namespace or all
        /// resources referenced by the ProtectedApplication) before restoring the
        /// resources from the Backup. This mode should only be used when you are
        /// intending to revert some portion of a cluster to an earlier state.
        DeleteAndRestore = 1,
        /// If conflicting top-level resources (either Namespaces or
        /// ProtectedApplications, depending upon the scope) are encountered at the
        /// beginning of a restore process, the Restore will fail.  If a conflict
        /// occurs during the restore process itself (e.g., because an out of band
        /// process creates conflicting resources), a conflict will be reported.
        FailOnConflict = 2,
    }
    /// Specifies the namespaced resources to restore from the Backup.
    /// Only one of the entries may be specified. If not specified, NO namespaced
    /// resources will be restored.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NamespacedResourceRestoreScope {
        /// Restore all namespaced resources in the Backup if set to "True".
        /// Specifying this field to "False" is an error.
        #[prost(bool, tag = "5")]
        AllNamespaces(bool),
        /// A list of selected Namespaces to restore from the Backup. The listed
        /// Namespaces and all resources contained in them will be restored.
        #[prost(message, tag = "6")]
        SelectedNamespaces(super::Namespaces),
        /// A list of selected ProtectedApplications to restore. The listed
        /// ProtectedApplications and all the resources to which they refer will be
        /// restored.
        #[prost(message, tag = "7")]
        SelectedApplications(super::NamespacedNames),
    }
}
/// Backup as stored in Platform log. It's used to log the details of
/// a createBackup/updateBackup request, so only fields that can be taken
/// from API calls are included here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggedBackup {
    /// A set of custom labels supplied by user.
    #[prost(btree_map = "string, string", tag = "1")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// delete_lock_days specifies the number of days from the create_time of this
    /// Backup before which deletion will be blocked.
    #[prost(int32, tag = "2")]
    pub delete_lock_days: i32,
    /// retain_days specifies the desired number of days from the create_time of
    /// this Backup after which it will be automatically deleted.
    #[prost(int32, tag = "3")]
    pub retain_days: i32,
    /// User specified descriptive string for this Backup.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Current state of the Backup
    #[prost(enumeration = "logged_backup::State", tag = "5")]
    pub state: i32,
    /// Human-readable description of why the backup is in the current `state`.
    #[prost(string, tag = "6")]
    pub state_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LoggedBackup`.
pub mod logged_backup {
    /// State
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The Backup resource is in the process of being created.
        Unspecified = 0,
        /// The Backup resource has been created and the associated BackupJob
        /// Kubernetes resource has been injected into the source cluster.
        Creating = 1,
        /// The gkebackup agent in the cluster has begun executing the backup
        /// operation.
        InProgress = 2,
        /// The backup operation has completed successfully.
        Succeeded = 3,
        /// The backup operation has failed.
        Failed = 4,
        /// This Backup resource (and its associated artifacts) is in the process
        /// of being deleted.
        Deleting = 5,
    }
}
/// BackupPlan as stored in Platform log. It's used to log the details of
/// a createBackupPlan/updateBackupPlan request, so only fields that can be taken
/// from user input are included here.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggedBackupPlan {
    /// User specified descriptive string for this BackupPlan.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// GCP resource name of the source cluster for this BackupPlan.
    #[prost(string, tag = "2")]
    pub cluster: ::prost::alloc::string::String,
    /// RetentionPolicy governs lifecycle of Backups created under this plan.
    #[prost(message, optional, tag = "3")]
    pub retention_policy: ::core::option::Option<logged_backup_plan::RetentionPolicy>,
    /// A set of custom labels supplied by user.
    #[prost(btree_map = "string, string", tag = "4")]
    pub labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Defines scheduled Backup creation under this BackupPlan.
    #[prost(message, optional, tag = "5")]
    pub backup_schedule: ::core::option::Option<logged_backup_plan::Schedule>,
    /// A flag indicates whether the plan has been deactivated.
    #[prost(bool, tag = "6")]
    pub deactivated: bool,
    /// Defines backup configuration of this BackupPlan.
    #[prost(message, optional, tag = "7")]
    pub backup_config: ::core::option::Option<logged_backup_plan::BackupConfig>,
}
/// Nested message and enum types in `LoggedBackupPlan`.
pub mod logged_backup_plan {
    /// RentionPolicy is an inner message type to define:
    /// 1. When to automatically delete Backups created under this BackupPlan
    /// 2. A plan level minimum Backup retain days which blocks deletion
    /// 3. Lock to disallow any policy updates
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetentionPolicy {
        /// Number of days during which deletion of a Backup created under this
        /// BackupPlan will be blocked.
        #[prost(int32, tag = "1")]
        pub backup_delete_lock_days: i32,
        /// Number of days after which the service will delete a Backup.
        /// If specified, a Backup created under this BackupPlan will be
        /// automatically deleted after its age reaches create_time +
        /// backup_retain_days.
        #[prost(int32, tag = "2")]
        pub backup_retain_days: i32,
        /// A flag denotes that the retention policy of this BackupPlan is locked.
        /// If set to True, no further update is allowed on this policy, including
        /// the 'locked' field itself.
        /// Default to False.
        #[prost(bool, tag = "3")]
        pub locked: bool,
    }
    /// Schedule, an inner message type defines a cron schedule.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schedule {
        /// A cron style string schedule on which an operation will be executed.
        #[prost(string, tag = "1")]
        pub cron_schedule: ::prost::alloc::string::String,
        /// A flag to toggle scheduled operation.
        #[prost(bool, tag = "2")]
        pub paused: bool,
    }
    /// BackupConfig, an inner message type defines the configuration of creating
    /// a backup from this BackupPlan
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BackupConfig {
        /// A boolean flag specifies whether volume data should be backed up
        #[prost(bool, tag = "4")]
        pub include_volume_data: bool,
        /// A boolean flag specifies whether secrets should be backed up
        #[prost(bool, tag = "5")]
        pub include_secrets: bool,
        /// Custom encryption key. For preview, support GCP KMS only.
        /// This only contains the key metadata, and no key material.
        #[prost(message, optional, tag = "6")]
        pub encryption_key: ::core::option::Option<super::EncryptionKey>,
        #[prost(oneof = "backup_config::BackupScope", tags = "1, 2, 3")]
        pub backup_scope: ::core::option::Option<backup_config::BackupScope>,
    }
    /// Nested message and enum types in `BackupConfig`.
    pub mod backup_config {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum BackupScope {
            /// If set to true, backup whole cluster
            #[prost(bool, tag = "1")]
            AllNamespaces(bool),
            /// If set, backup the list of namespaces
            #[prost(message, tag = "2")]
            SelectedNamespaces(super::super::Namespaces),
            /// If set, backup the list of applications
            #[prost(message, tag = "3")]
            SelectedApplications(super::super::NamespacedNames),
        }
    }
}
/// use case 1
/// A log entry when modification(creation, update, deletion) is made to a
/// BackupPlan
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupPlanChange {
    /// The full name of the old BackupPlan resource that is being modified.
    /// Empty for creation.
    /// Format: projects/{project}/locations/{location}/backupPlans/{backup_plan}
    #[prost(string, tag = "1")]
    pub backup_plan: ::prost::alloc::string::String,
    /// Type of the change is being made.
    #[prost(enumeration = "ChangeType", tag = "2")]
    pub change_type: i32,
    /// Modification details.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The input BackupPlan resource with the updated fields populated to update
    /// the source BackupPlan to.
    #[prost(message, optional, tag = "4")]
    pub input_backup_plan: ::core::option::Option<LoggedBackupPlan>,
    /// The error code and message.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// use case 2
/// A log entry when modification(creation, update, deletion) is made to a
/// Backup
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupChange {
    /// The full name of the Backup resource that is being modified.
    /// Format:
    /// projects/{project}/locations/{location}/backupPlans/{backup_plan}/backups/{backup}
    #[prost(string, tag = "1")]
    pub backup: ::prost::alloc::string::String,
    /// Type of the change is being made.
    #[prost(enumeration = "ChangeType", tag = "2")]
    pub change_type: i32,
    /// Whether the change is made manually or automatically.
    #[prost(bool, tag = "3")]
    pub scheduled: bool,
    /// Modification details.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The input Backup resource with the updated fields populated to update
    /// the source Backup to, or the backup created automatically from retention
    /// policy.
    #[prost(message, optional, tag = "5")]
    pub input_backup: ::core::option::Option<LoggedBackup>,
    /// The error code and message.
    #[prost(message, optional, tag = "6")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// use case 3
/// A log entry when modification(creation, update, deletion) is made to a
/// restorePlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestorePlanChange {
    /// The full name of the RestorePlan resource that is being modified.
    /// Empty for creation.
    /// Format: projects/*/locations/*/restorePlans/*
    #[prost(string, tag = "1")]
    pub restore_plan: ::prost::alloc::string::String,
    /// Type of the change is being made.
    #[prost(enumeration = "ChangeType", tag = "2")]
    pub change_type: i32,
    /// Modification details.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The input RestorePlan resource with the updated fields populated to update
    /// the source RestorePlan to.
    #[prost(message, optional, tag = "4")]
    pub input_restore_plan: ::core::option::Option<LoggedRestorePlan>,
    /// The error code and message.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// use case 4
/// A log entry when modification(creation, update, deletion) is made to a
/// restore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreChange {
    /// The full name of the Restore resource that is being modified.
    /// Empty for creation.
    /// Format: projects/*/locations/*/restorePlans/*/restores/*
    #[prost(string, tag = "1")]
    pub restore: ::prost::alloc::string::String,
    /// Type of the change is being made.
    #[prost(enumeration = "ChangeType", tag = "2")]
    pub change_type: i32,
    /// Modification details.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The input Restore resource with the updated fields populated to update
    /// the source Restore to.
    #[prost(message, optional, tag = "4")]
    pub input_restore: ::core::option::Option<LoggedRestore>,
    /// The error code and message.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
}
/// The type of changes this log is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeType {
    /// Default value, not specified.
    Unspecified = 0,
    /// The resource is created.
    Creation = 1,
    /// The resource is updated.
    Update = 2,
    /// The resource is deleted.
    Deletion = 3,
}
