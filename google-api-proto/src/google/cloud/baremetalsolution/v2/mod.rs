/// A storage volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Output only. The resource name of this `Volume`.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// Format:
    /// `projects/{project}/locations/{location}/volumes/{volume}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The storage type for this volume.
    #[prost(enumeration="volume::StorageType", tag="2")]
    pub storage_type: i32,
    /// The state of this storage volume.
    #[prost(enumeration="volume::State", tag="3")]
    pub state: i32,
    /// The requested size of this storage volume, in GiB.
    #[prost(int64, tag="4")]
    pub requested_size_gib: i64,
    /// The current size of this storage volume, in GiB, including space reserved
    /// for snapshots. This size might be different than the requested size if the
    /// storage volume has been configured with auto grow or auto shrink.
    #[prost(int64, tag="5")]
    pub current_size_gib: i64,
    /// The size, in GiB, that this storage volume has expanded as a result of an
    /// auto grow policy. In the absence of auto-grow, the value is 0.
    #[prost(int64, tag="6")]
    pub auto_grown_size_gib: i64,
    /// The space remaining in the storage volume for new LUNs, in GiB, excluding
    /// space reserved for snapshots.
    #[prost(int64, tag="7")]
    pub remaining_space_gib: i64,
    /// Details about snapshot space reservation and usage on the storage volume.
    #[prost(message, optional, tag="8")]
    pub snapshot_reservation_detail: ::core::option::Option<volume::SnapshotReservationDetail>,
    /// The behavior to use when snapshot reserved space is full.
    #[prost(enumeration="volume::SnapshotAutoDeleteBehavior", tag="9")]
    pub snapshot_auto_delete_behavior: i32,
    /// The name of the snapshot schedule policy in use for this volume, if any.
    #[prost(string, tag="10")]
    pub snapshot_schedule_policy: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    /// Details about snapshot space reservation and usage on the storage volume.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SnapshotReservationDetail {
        /// The space on this storage volume reserved for snapshots, shown in GiB.
        #[prost(int64, tag="1")]
        pub reserved_space_gib: i64,
        /// The percent of snapshot space on this storage volume actually being used
        /// by the snapshot copies. This value might be higher than 100% if the
        /// snapshot copies have overflowed into the data portion of the storage
        /// volume.
        #[prost(int32, tag="2")]
        pub reserved_space_used_percent: i32,
        /// The amount, in GiB, of available space in this storage volume's reserved
        /// snapshot space.
        #[prost(int64, tag="3")]
        pub reserved_space_remaining_gib: i64,
    }
    /// The storage type for a volume.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StorageType {
        /// The storage type for this volume is unknown.
        Unspecified = 0,
        /// The storage type for this volume is SSD.
        Ssd = 1,
        /// This storage type for this volume is HDD.
        Hdd = 2,
    }
    /// The possible states for a storage volume.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The storage volume is in an unknown state.
        Unspecified = 0,
        /// The storage volume is being created.
        Creating = 1,
        /// The storage volume is ready for use.
        Ready = 2,
        /// The storage volume has been requested to be deleted.
        Deleting = 3,
    }
    /// The kinds of auto delete behavior to use when snapshot reserved space is
    /// full.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SnapshotAutoDeleteBehavior {
        /// The unspecified behavior.
        Unspecified = 0,
        /// Don't delete any snapshots. This disables new snapshot creation, as
        /// long as the snapshot reserved space is full.
        Disabled = 1,
        /// Delete the oldest snapshots first.
        OldestFirst = 2,
        /// Delete the newest snapshots first.
        NewestFirst = 3,
    }
}
/// Message for requesting a list of storage volumes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumesRequest {
    /// Required. Parent value for ListVolumesRequest.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message containing the list of storage volumes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumesResponse {
    /// The list of storage volumes.
    #[prost(message, repeated, tag="1")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for requesting storage volume information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVolumeRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A storage volume logical unit number (LUN).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lun {
    /// Output only. The name of the LUN.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The state of this storage volume.
    #[prost(enumeration="lun::State", tag="2")]
    pub state: i32,
    /// The size of this LUN, in gigabytes.
    #[prost(int64, tag="3")]
    pub size_gb: i64,
    /// The LUN multiprotocol type ensures the characteristics of the LUN are
    /// optimized for each operating system.
    #[prost(enumeration="lun::MultiprotocolType", tag="4")]
    pub multiprotocol_type: i32,
    /// Display the storage volume for this LUN.
    #[prost(string, tag="5")]
    pub storage_volume: ::prost::alloc::string::String,
    /// Display if this LUN can be shared between multiple physical servers.
    #[prost(bool, tag="6")]
    pub shareable: bool,
    /// Display if this LUN is a boot LUN.
    #[prost(bool, tag="7")]
    pub boot_lun: bool,
    /// The storage type for this LUN.
    #[prost(enumeration="lun::StorageType", tag="8")]
    pub storage_type: i32,
    /// The WWID for this LUN.
    #[prost(string, tag="9")]
    pub wwid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Lun`.
pub mod lun {
    /// The possible states for the LUN.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The LUN is in an unknown state.
        Unspecified = 0,
        /// The LUN is being created.
        Creating = 1,
        /// The LUN is being updated.
        Updating = 2,
        /// The LUN is ready for use.
        Ready = 3,
        /// The LUN has been requested to be deleted.
        Deleting = 4,
    }
    /// Display the operating systems present for the LUN multiprotocol type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MultiprotocolType {
        /// Server has no OS specified.
        Unspecified = 0,
        /// Server with Linux OS.
        Linux = 1,
    }
    /// The storage types for a LUN.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StorageType {
        /// The storage type for this LUN is unknown.
        Unspecified = 0,
        /// This storage type for this LUN is SSD.
        Ssd = 1,
        /// This storage type for this LUN is HDD.
        Hdd = 2,
    }
}
/// A Network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// Output only. The resource name of this `Network`.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// Format:
    /// `projects/{project}/locations/{location}/networks/{network}`
    /// This field will contain the same value as field "network", which will soon
    /// be deprecated. Please use this field to reference the name of the network
    /// resource.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// Name of the network.
    #[prost(string, tag="1")]
    pub network: ::prost::alloc::string::String,
    /// The type of this network.
    #[prost(enumeration="network::Type", tag="2")]
    pub r#type: i32,
    /// IP address configured.
    #[prost(string, tag="3")]
    pub ip_address: ::prost::alloc::string::String,
    /// List of physical interfaces.
    #[prost(string, repeated, tag="4")]
    pub mac_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Network state.
    #[prost(enumeration="network::State", tag="6")]
    pub state: i32,
    /// The vlan id of the Network.
    #[prost(string, tag="7")]
    pub vlan_id: ::prost::alloc::string::String,
    /// The cidr of the Network.
    #[prost(string, tag="8")]
    pub cidr: ::prost::alloc::string::String,
    /// The vrf for the Network.
    #[prost(message, optional, tag="9")]
    pub vrf: ::core::option::Option<Vrf>,
}
/// Nested message and enum types in `Network`.
pub mod network {
    /// Network type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified value.
        Unspecified = 0,
        /// Client network, a network peered to a Google Cloud VPC.
        Client = 1,
        /// Private network, a network local to the Bare Metal Solution environment.
        Private = 2,
    }
    /// The possible states for this Network.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The Network is in an unknown state.
        Unspecified = 0,
        /// The Network is provisioning.
        Provisioning = 1,
        /// The Network has been provisioned.
        Provisioned = 2,
    }
}
/// A network VRF.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vrf {
    /// The name of the VRF.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The possible state of VRF.
    #[prost(enumeration="vrf::State", tag="5")]
    pub state: i32,
    /// The QOS policy applied to this VRF.
    #[prost(message, optional, tag="6")]
    pub qos_policy: ::core::option::Option<vrf::QosPolicy>,
    /// The list of VLAN attachments for the VRF.
    #[prost(message, repeated, tag="7")]
    pub vlan_attachments: ::prost::alloc::vec::Vec<vrf::VlanAttachment>,
}
/// Nested message and enum types in `VRF`.
pub mod vrf {
    /// QOS policy parameters.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QosPolicy {
        /// The bandwidth permitted by the QOS policy, in gbps.
        #[prost(double, tag="1")]
        pub bandwidth_gbps: f64,
    }
    /// VLAN attachment details.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VlanAttachment {
        /// The peer vlan ID of the attachment.
        #[prost(int64, tag="1")]
        pub peer_vlan_id: i64,
        /// The peer IP of the attachment.
        #[prost(string, tag="2")]
        pub peer_ip: ::prost::alloc::string::String,
        /// The router IP of the attachment.
        #[prost(string, tag="3")]
        pub router_ip: ::prost::alloc::string::String,
    }
    /// The possible states for this VRF.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The unspecified state.
        Unspecified = 0,
        /// The vrf is provisioning.
        Provisioning = 1,
        /// The vrf is provisioned.
        Provisioned = 2,
    }
}
/// Message for requesting a list of networks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworksRequest {
    /// Required. Parent value for ListNetworksRequest.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message containing the list of networks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNetworksResponse {
    /// The list of networks.
    #[prost(message, repeated, tag="1")]
    pub networks: ::prost::alloc::vec::Vec<Network>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for requesting network information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting snapshot schedule policy information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotSchedulePolicyRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The resource name of this `Instance`.
    /// Resource names are schemeless URIs that follow the conventions in
    /// <https://cloud.google.com/apis/design/resource_names.>
    /// Format:
    /// `projects/{project}/locations/{location}/instances/{instance}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Create a time stamp.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update a time stamp.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The server type.
    /// [Available server
    /// types](<https://cloud.google.com/bare-metal/docs/bms-planning#server_configurations>)
    #[prost(string, tag="4")]
    pub machine_type: ::prost::alloc::string::String,
    /// The state of the server.
    #[prost(enumeration="instance::State", tag="5")]
    pub state: i32,
    /// True if you enable hyperthreading for the server, otherwise false.
    /// The default value is false.
    #[prost(bool, tag="6")]
    pub hyperthreading_enabled: bool,
    /// Labels as key value pairs.
    #[prost(btree_map="string, string", tag="7")]
    pub labels: ::prost::alloc::collections::BTreeMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// List of LUNs associated with this server.
    #[prost(message, repeated, tag="8")]
    pub luns: ::prost::alloc::vec::Vec<Lun>,
    /// List of networks associated with this server.
    #[prost(message, repeated, tag="9")]
    pub networks: ::prost::alloc::vec::Vec<Network>,
    /// True if the interactive serial console feature is enabled for the instance,
    /// false otherwise.
    /// The default value is false.
    #[prost(bool, tag="10")]
    pub interactive_serial_console_enabled: bool,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// The possible states for this server.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The server is in an unknown state.
        Unspecified = 0,
        /// The server is being provisioned.
        Provisioning = 1,
        /// The server is running.
        Running = 2,
        /// The server has been deleted.
        Deleted = 3,
    }
}
/// Message for requesting the list of servers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Parent value for ListInstancesRequest.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for the list of servers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of servers.
    #[prost(message, repeated, tag="1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for requesting server information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Message requesting to reset a server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message from resetting a server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceResponse {
}
/// Message for requesting storage volume snapshot information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVolumeSnapshotRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting a list of storage volume snapshots.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeSnapshotsRequest {
    /// Required. Parent value for ListVolumesRequest.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message containing the list of storage volume snapshots.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeSnapshotsResponse {
    /// The list of storage volumes.
    #[prost(message, repeated, tag="1")]
    pub volume_snapshots: ::prost::alloc::vec::Vec<VolumeSnapshot>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for deleting named Volume snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVolumeSnapshotRequest {
    /// Required. The name of the snapshot to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata from a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the action executed by the operation.
    #[prost(string, tag="4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag="5")]
    pub status_message: ::prost::alloc::string::String,
    /// Identifies whether the user requested the cancellation
    /// of the operation. Operations that have been successfully cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag="6")]
    pub requested_cancellation: bool,
    /// API version used with the operation.
    #[prost(string, tag="7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Snapshot registered for a given storage volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeSnapshot {
    /// Output only. The name of the storage volume snapshot.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the storage volume snapshot.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The size of the storage volume snapshot, in bytes.
    #[prost(int64, tag="3")]
    pub size_bytes: i64,
    /// Optional. The creation time of the storage volume snapshot.
    #[prost(message, optional, tag="4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The storage volume this snapshot belongs to.
    #[prost(string, tag="5")]
    pub storage_volume: ::prost::alloc::string::String,
}
/// A snapshot schedule policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotSchedulePolicy {
    /// Output only. The name of the snapshot schedule policy.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the snapshot schedule policy.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// The snapshot schedules contained in this policy. You can specify a maximum
    ///  of 5 schedules.
    #[prost(message, repeated, tag="3")]
    pub schedules: ::prost::alloc::vec::Vec<snapshot_schedule_policy::Schedule>,
}
/// Nested message and enum types in `SnapshotSchedulePolicy`.
pub mod snapshot_schedule_policy {
    /// A snapshot schedule.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schedule {
        /// A crontab-like specification that the schedule uses to take
        /// snapshots.
        #[prost(string, tag="1")]
        pub crontab_spec: ::prost::alloc::string::String,
        /// The maximum number of snapshots to retain in this schedule.
        #[prost(int32, tag="2")]
        pub retention_count: i32,
        /// A list of snapshot names created in this schedule.
        #[prost(string, tag="3")]
        pub prefix: ::prost::alloc::string::String,
    }
}
/// Message for requesting a list of snapshot schedule policies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotSchedulePoliciesRequest {
    /// Required. The parent project containing the Snapshot Schedule Policies.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message containing the list of snapshot schedule policies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotSchedulePoliciesResponse {
    /// The snapshot schedule policies registered in this project.
    #[prost(message, repeated, tag="1")]
    pub snapshot_schedule_policies: ::prost::alloc::vec::Vec<SnapshotSchedulePolicy>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for creating a snapshot schedule policy in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSnapshotSchedulePolicyRequest {
    /// Required. The parent project and location containing the SnapshotSchedulePolicy.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The SnapshotSchedulePolicy to create.
    #[prost(message, optional, tag="2")]
    pub snapshot_schedule_policy: ::core::option::Option<SnapshotSchedulePolicy>,
    /// Required. Snapshot policy ID
    #[prost(string, tag="3")]
    pub snapshot_schedule_policy_id: ::prost::alloc::string::String,
}
/// Message for updating a snapshot schedule policy in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSnapshotSchedulePolicyRequest {
    /// Required. The snapshot schedule policy to update.
    ///
    /// The `name` field is used to identify the snapshot schedule policy to
    /// update. Format:
    /// projects/{project}/locations/global/snapshotSchedulePolicies/{policy}
    #[prost(message, optional, tag="1")]
    pub snapshot_schedule_policy: ::core::option::Option<SnapshotSchedulePolicy>,
    /// Required. The list of fields to update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Message for deleting a snapshot schedule policy in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotSchedulePolicyRequest {
    /// Required. The name of the snapshot schedule policy to delete.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for updating a volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVolumeRequest {
    /// Required. The volume to update.
    ///
    /// The `name` field is used to identify the volume to update.
    /// Format: projects/{project}/locations/{location}/volumes/{volume}
    #[prost(message, optional, tag="1")]
    pub volume: ::core::option::Option<Volume>,
    /// The list of fields to update.
    /// The only currently supported fields are:
    ///   `snapshot_auto_delete_behavior`
    ///   `snapshot_schedule_policy_name`
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Message for requesting storage lun information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLunRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting a list of storage volume luns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLunsRequest {
    /// Required. Parent value for ListLunsRequest.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server might return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message containing the list of storage volume luns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLunsResponse {
    /// The list of luns.
    #[prost(message, repeated, tag="1")]
    pub luns: ::prost::alloc::vec::Vec<Lun>,
    /// A token identifying a page of results from the server.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag="3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a volume snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVolumeSnapshotRequest {
    /// Required. The volume to snapshot.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The volume snapshot to create. Only the description field may be specified.
    #[prost(message, optional, tag="2")]
    pub volume_snapshot: ::core::option::Option<VolumeSnapshot>,
}
/// Message for restoring a volume snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreVolumeSnapshotRequest {
    /// Required. Name of the resource.
    #[prost(string, tag="1")]
    pub volume_snapshot: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod bare_metal_solution_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Performs management operations on Bare Metal Solution servers.
    ///
    /// The `baremetalsolution.googleapis.com` service provides management
    /// capabilities for Bare Metal Solution servers. To access the API methods, you
    /// must assign Bare Metal Solution IAM roles containing the desired permissions
    /// to your staff in your Google Cloud project. You must also enable the Bare
    /// Metal Solution API. Once enabled, the methods act
    /// upon specific servers in your Bare Metal Solution environment.
    #[derive(Debug, Clone)]
    pub struct BareMetalSolutionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BareMetalSolutionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BareMetalSolutionClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BareMetalSolutionClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// List servers in a given project and location.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get details about a single server.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Perform an ungraceful, hard reset on a server. Equivalent to shutting the
        /// power off and then turning it back on.
        pub async fn reset_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetInstanceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ResetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List storage volumes in a given project and location.
        pub async fn list_volumes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVolumesRequest>,
        ) -> Result<tonic::Response<super::ListVolumesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListVolumes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get details of a single storage volume.
        pub async fn get_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVolumeRequest>,
        ) -> Result<tonic::Response<super::Volume>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetVolume",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update details of a single storage volume.
        pub async fn update_volume(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVolumeRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/UpdateVolume",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List network in a given project and location.
        pub async fn list_networks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNetworksRequest>,
        ) -> Result<tonic::Response<super::ListNetworksResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListNetworks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get details of a single network.
        pub async fn get_network(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkRequest>,
        ) -> Result<tonic::Response<super::Network>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetNetwork",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List snapshot schedule policies in a given project and location.
        pub async fn list_snapshot_schedule_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotSchedulePoliciesRequest>,
        ) -> Result<
                tonic::Response<super::ListSnapshotSchedulePoliciesResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListSnapshotSchedulePolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get details of a single snapshot schedule policy.
        pub async fn get_snapshot_schedule_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotSchedulePolicyRequest>,
        ) -> Result<tonic::Response<super::SnapshotSchedulePolicy>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetSnapshotSchedulePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a snapshot schedule policy in the specified project.
        pub async fn create_snapshot_schedule_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSnapshotSchedulePolicyRequest>,
        ) -> Result<tonic::Response<super::SnapshotSchedulePolicy>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/CreateSnapshotSchedulePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a snapshot schedule policy in the specified project.
        pub async fn update_snapshot_schedule_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSnapshotSchedulePolicyRequest>,
        ) -> Result<tonic::Response<super::SnapshotSchedulePolicy>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/UpdateSnapshotSchedulePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a named snapshot schedule policy.
        pub async fn delete_snapshot_schedule_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSnapshotSchedulePolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/DeleteSnapshotSchedulePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a storage volume snapshot in a containing volume.
        pub async fn create_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVolumeSnapshotRequest>,
        ) -> Result<tonic::Response<super::VolumeSnapshot>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/CreateVolumeSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Restore a storage volume snapshot to its containing volume.
        pub async fn restore_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreVolumeSnapshotRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/RestoreVolumeSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a storage volume snapshot for a given volume.
        pub async fn delete_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVolumeSnapshotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/DeleteVolumeSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get details of a single storage volume snapshot.
        pub async fn get_volume_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVolumeSnapshotRequest>,
        ) -> Result<tonic::Response<super::VolumeSnapshot>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetVolumeSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List storage volume snapshots for given storage volume.
        pub async fn list_volume_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVolumeSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListVolumeSnapshotsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListVolumeSnapshots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get details of a single storage logical unit number(LUN).
        pub async fn get_lun(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLunRequest>,
        ) -> Result<tonic::Response<super::Lun>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/GetLun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List storage volume luns for given storage volume.
        pub async fn list_luns(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLunsRequest>,
        ) -> Result<tonic::Response<super::ListLunsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.baremetalsolution.v2.BareMetalSolution/ListLuns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
