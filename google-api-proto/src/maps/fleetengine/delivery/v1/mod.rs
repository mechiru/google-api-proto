/// DeliveryVehicle metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryVehicle {
    /// The unique name for this vehicle.
    /// The format is providers/{provider}/deliveryVehicles/{vehicle}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The last reported location of the vehicle.
    #[prost(message, optional, tag = "2")]
    pub last_location: ::core::option::Option<super::super::v1::VehicleLocation>,
    /// The Vehicle's navigation status.
    #[prost(enumeration = "super::super::v1::NavigationStatus", tag = "3")]
    pub navigation_status: i32,
    /// The encoded polyline specifying the route the navigation recommends taking
    /// to the next waypoint. Your driver app updates this every time a stop is
    /// reached or passed, or the navigation reroutes. These LatLngs are returned
    /// in `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\].path`
    /// for all active tasks assigned to the vehicle.
    ///
    /// There are a few cases where this field may not be used to populate
    /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\].path`:
    ///
    /// 1. The endpoint of the `current_route_segment` does not match
    /// `DeliveryVehicle.remaining_vehicle_journey_segments\[0\].stop`.
    ///
    /// 2. The driver app has not updated its location recently, so the last
    /// updated value for this field may be stale.
    ///
    /// 3. The driver app has recently updated its location, but the
    /// `current_route_segment` is stale and points to a previous vehicle stop.
    ///
    /// In these cases, Fleet Engine will populate this field with a
    /// route from the most recently passed VehicleStop to the upcoming VehicleStop
    /// to ensure the consumer of this field has the best available information on
    /// the current path of the delivery vehicle.
    #[prost(bytes = "bytes", tag = "4")]
    pub current_route_segment: ::prost::bytes::Bytes,
    /// The location where the `current_route_segment` ends. This is not currently
    /// populated by the driver app, but it can be supplied on
    /// UpdateDeliveryVehicle calls as the latlng from the upcoming vehicle stop or
    /// the last latlng of the `current_route_segment`. Fleet Engine will then do
    /// its best to interpolate to an actual VehicleStop.
    ///
    /// This field is ignored in UpdateDeliveryVehicle calls if the
    /// `DeliveryVehicle.current_route_segment` field is empty.
    #[prost(message, optional, tag = "5")]
    pub current_route_segment_end_point:
        ::core::option::Option<super::super::super::super::google::r#type::LatLng>,
    /// The remaining driving distance for the `current_route_segment`.
    /// This value is usually updated by the driver app because it is considered to
    /// have more accurate information about the current route than Fleet Engine.
    /// However, it may be populated by Fleet Engine. For more information, see the
    /// documentation for `DeliveyVehicle.current_route_segment'. This field is
    /// returned in
    /// `Task.remaining_vehicle_journey_segment\[0\].driving_distance_meters` for all
    /// active tasks assigned to the vehicle.
    ///
    /// This field is ignored in UpdateDeliveryVehicle calls if the
    /// `DeliveryVehicle.current_route_segment` field is empty.
    #[prost(message, optional, tag = "6")]
    pub remaining_distance_meters: ::core::option::Option<i32>,
    /// The remaining driving time for the `current_route_segment`.
    /// This value is usually updated by the driver app because it is considered to
    /// have more accurate information about the current route than Fleet Engine.
    /// However, it may be populated by Fleet Engine. For more information, see the
    /// documentation for `DeliveyVehicle.current_route_segment'. This field is
    /// returned in `Task.remaining_vehicle_journey_segment\[0\].driving_duration`
    /// for all active tasks assigned to the vehicle.
    ///
    /// This field is ignored in UpdateDeliveryVehicle calls if the
    /// `DeliveryVehicle.current_route_segment` field is empty.
    #[prost(message, optional, tag = "7")]
    pub remaining_duration: ::core::option::Option<::prost_types::Duration>,
    /// The journey segments assigned to this vehicle, starting from the vehicle's
    /// most recently reported location.
    #[prost(message, repeated, tag = "8")]
    pub remaining_vehicle_journey_segments: ::prost::alloc::vec::Vec<VehicleJourneySegment>,
    /// List of custom delivery vehicle attributes.
    /// Each attribute has a unique key.
    #[prost(message, repeated, tag = "9")]
    pub attributes: ::prost::alloc::vec::Vec<super::super::v1::VehicleAttribute>,
}
/// A location with any additional identifiers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// The lat/lng of the location.
    #[prost(message, optional, tag = "1")]
    pub point: ::core::option::Option<super::super::super::super::google::r#type::LatLng>,
}
/// Represents a vehicle’s travel segment from its previous stop to the
/// current stop. If it is the first active stop, then it is from
/// Vehicle’s current location to this stop.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleJourneySegment {
    /// Specifies the actual stop location and the tasks associated with
    /// the stop. Some fields of the VehicleStop may not be present if this journey
    /// segment is part of JourneySharingInfo.
    #[prost(message, optional, tag = "1")]
    pub stop: ::core::option::Option<VehicleStop>,
    /// Output only. The travel distance from the previous stop to this stop.
    /// If the current stop is the first stop in the list of journey
    /// segments, then the starting point is the vehicle's location recorded
    /// at the time that this stop was added to the list. This field may not be
    /// present if this journey segment is part of JourneySharingInfo.
    #[prost(message, optional, tag = "2")]
    pub driving_distance_meters: ::core::option::Option<i32>,
    /// Output only. The travel time from the previous stop to this stop.
    /// If the current stop is the first stop in the list of journey
    /// segments, then the starting point is the vehicle's location recorded
    /// at the time that this stop was added to the list.
    ///
    /// If this field is defined in the path
    /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\].driving_duration`,
    /// then it may be populated with the value from
    /// `DeliveryVehicle.remaining_duration` so it provides the remaining driving
    /// duration from the driver app's latest known location, not the driving time
    /// from the previous stop.
    #[prost(message, optional, tag = "3")]
    pub driving_duration: ::core::option::Option<::prost_types::Duration>,
    /// The path from the previous stop to this stop. If the current stop is the
    /// first stop in the list of journey segments, then this is the path from the
    /// vehicle's current location to this stop at the time that the stop was
    /// added to the list. This field may not be present if this journey segment is
    /// part of JourneySharingInfo.
    ///
    /// If this field is defined in the path
    /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\].path`,
    /// then it may be populated with the LatLngs decoded from
    /// `DeliveryVehicle.current_route_segment` so it provides the driving
    /// path from the driver app's latest known location, not the path from
    /// the previous stop.
    #[prost(message, repeated, tag = "5")]
    pub path: ::prost::alloc::vec::Vec<super::super::super::super::google::r#type::LatLng>,
}
/// Describes a point where a vehicle will stop on its journey to perform
/// one or more tasks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleStop {
    /// Required. The location of the stop. Note that the locations in the tasks may not
    /// exactly match this location, but will be within a short distance.
    /// This field will not be present when returned as part of GetTaskResponse and
    /// SearchTasksResponse.
    #[prost(message, optional, tag = "1")]
    pub planned_location: ::core::option::Option<LocationInfo>,
    /// The list of tasks to be performed at this stop. Some fields of TaskInfo
    /// will not be present when returned as part of GetTaskResponse or
    /// SearchTasksResponse.
    #[prost(message, repeated, tag = "2")]
    pub tasks: ::prost::alloc::vec::Vec<vehicle_stop::TaskInfo>,
    /// The state of the VehicleStop. This field will not be present when returned
    /// as part of GetTaskResponse or SearchTasksResponse.
    #[prost(enumeration = "vehicle_stop::State", tag = "3")]
    pub state: i32,
}
/// Nested message and enum types in `VehicleStop`.
pub mod vehicle_stop {
    /// Additional information about the task performed at this stop.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TaskInfo {
        /// The task ID. This field will not be present when returned as part
        /// of GetTaskResponse or SearchTasksResponse.
        #[prost(string, tag = "1")]
        pub task_id: ::prost::alloc::string::String,
        /// The time required to perform the task.
        #[prost(message, optional, tag = "2")]
        pub task_duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// The current state of a VehicleStop.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unknown.
        Unspecified = 0,
        /// Created but not actively routing to.
        New = 1,
        /// Assigned and actively routing to.
        Enroute = 2,
        /// Arrived at stop. Assumes that when the vehicle is routing to the next
        /// stop, all previous stops are completed.
        Arrived = 3,
    }
}
/// A task in the Delivery API represents a single action to track.
/// In general there is a distinction between shipment-related tasks and break
/// tasks. A shipment can have multiple tasks associated with it; for
/// example, one task for the pickup and one for the dropoff or transfer.
/// Different tasks for a given shipment can be handled by different vehicles;
/// for example, one vehicle handles the pickup and drives the shipment to the
/// hub, while another drives the same shipment from the hub to the dropoff.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// In the format `providers/{provider_id}/tasks/{task_id}`. The task_id must
    /// be a unique identifier and not a `tracking_id`. To store a `tracking_id` of
    /// a shipment use the `tracking_id` field. Multiple tasks can have the same
    /// `tracking_id`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. Defines the type of the task; for example, a break or shipment.
    #[prost(enumeration = "task::Type", tag = "2")]
    pub r#type: i32,
    /// Required. The current execution state of the task.
    #[prost(enumeration = "task::State", tag = "3")]
    pub state: i32,
    /// The outcome of the task.
    #[prost(enumeration = "task::TaskOutcome", tag = "9")]
    pub task_outcome: i32,
    /// The timestamp of when the task's outcome was set (from provider).
    #[prost(message, optional, tag = "10")]
    pub task_outcome_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Location where the task's outcome was set. Updated as part of UpdateTask.
    /// redacted as part of SearchTasks requests. If not explicitly updated by
    /// provider then Fleet Engine will populate it by default with the last known
    /// vehicle location (raw location).
    #[prost(message, optional, tag = "11")]
    pub task_outcome_location: ::core::option::Option<LocationInfo>,
    /// Indicates where the value of the `task_outcome_location` came from.
    #[prost(enumeration = "task::TaskOutcomeLocationSource", tag = "12")]
    pub task_outcome_location_source: i32,
    /// Immutable. This field facilitates the storing of an ID for the customer to avoid
    /// unnecessary or complicated mapping. Cannot be set for Tasks of type
    /// `UNAVAILABLE` or `SCHEDULED_STOP`. IDs are subject to the
    /// following normalization and restrictions:
    ///
    /// 1. IDs must be valid Unicode strings.
    /// 2. IDs are limited to a maximum length of 64 characters.
    /// 3. IDs will be normalized according to Unicode Normalization Form C
    /// (<http://www.unicode.org/reports/tr15/>).
    /// 4. IDs may not contain any of the following ASCII characters: '/', ':',
    /// '\\', '?', or '#'.
    #[prost(string, tag = "4")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Output only. The ID of the vehicle making this Task.
    #[prost(string, tag = "5")]
    pub delivery_vehicle_id: ::prost::alloc::string::String,
    /// Immutable. The location where the task is to be completed.
    /// Optional for `UNAVAILABLE` Tasks, required for all others.
    #[prost(message, optional, tag = "6")]
    pub planned_location: ::core::option::Option<LocationInfo>,
    /// Required. Immutable. Additional time to perform an action at this location.
    #[prost(message, optional, tag = "7")]
    pub task_duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Journey sharing specific fields. Not populated when state is `CLOSED`.
    #[prost(message, optional, tag = "8")]
    pub journey_sharing_info: ::core::option::Option<task::JourneySharingInfo>,
}
/// Nested message and enum types in `Task`.
pub mod task {
    /// Journey sharing specific fields.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JourneySharingInfo {
        /// Tracking information for each stop that the assigned vehicle will
        /// travel to before completing this task. This list may contain stops
        /// from other tasks.
        ///
        /// The first segment,
        /// `Task.journey_sharing_info.remaining_vehicle_journey_segments\[0\]`,
        /// contains route information from the driver's last known location to the
        /// upcoming VehicleStop. This current route information usually comes from
        /// the driver app except for some cases noted in the documentation for
        /// `DeliveyVehicle.current_route_segment`. The other segments in
        /// `Task.journey_sharing_info.remaining_vehicle_journey_segments` are
        /// populated by Fleet Engine and provide route information between the
        /// remaining VehicleStops.
        #[prost(message, repeated, tag = "1")]
        pub remaining_vehicle_journey_segments:
            ::prost::alloc::vec::Vec<super::VehicleJourneySegment>,
        /// Indicates the last reported location of the assigned vehicle
        /// along the route.
        #[prost(message, optional, tag = "2")]
        pub last_location: ::core::option::Option<super::super::super::v1::VehicleLocation>,
        /// Indicates whether the vehicle's lastLocation can be snapped to
        /// the `current_route_segment`. False if `last_location` or
        /// `current_route_segment` do not exist. This value is computed by
        /// Fleet Engine. Any update from clients will be ignored.
        #[prost(bool, tag = "3")]
        pub last_location_snappable: bool,
    }
    /// The type of a Task.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default, the task type is not known.
        Unspecified = 0,
        /// A pickup task is the action taken for picking up a shipment from an end
        /// customer. Depot or feeder vehicle pickups should use the `SCHEDULED_STOP`
        /// type.
        Pickup = 1,
        /// A delivery task is the action taken for delivering a shipment to an end
        /// customer. Depot or feeder vehicle dropoffs should use the
        /// `SCHEDULED_STOP` type.
        Delivery = 2,
        /// A scheduled stop task is used for planning purposes. For example, it may
        /// represent picking up or dropping off shipments at feeder vehicles or
        /// depots. It should not be used for any shipments that are picked up or
        /// dropped off from an end customer.
        ScheduledStop = 3,
        /// A task that indicates unavailability (e.g. driver breaks or vehicle
        /// refueling).
        Unavailable = 4,
    }
    /// The state of a Task indicating its progression.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default, used for an unspecified or unrecognized Task state.
        Unspecified = 0,
        /// The task has not yet been assigned a delivery vehicle, or the delivery
        /// vehicle has not yet passed the task's assigned vehicle stop.
        Open = 1,
        /// When the vehicle this task was assigned to passes the vehicle stop of
        /// this task.
        Closed = 2,
    }
    /// The outcome of attempting to execute a task. When TaskState is closed,
    /// this indicates whether it was completed successfully or not.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TaskOutcome {
        /// Task outcome before being set.
        Unspecified = 0,
        /// Task was completed successfully.
        Succeeded = 1,
        /// Task could not be completed or was cancelled.
        Failed = 2,
    }
    /// The identity of the source which populated the task_outcome_location.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TaskOutcomeLocationSource {
        /// Task outcome before being set.
        Unspecified = 0,
        /// The provider specified the task_outcome_location.
        Provider = 2,
        /// The provider did not specify the task_outcome_location so Fleet Engine
        /// used the last known vehicle location.
        LastVehicleLocation = 3,
    }
}
/// CreateDeliveryVehicle request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeliveryVehicleRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DeliveryVehicle ID must be unique for each provider. IDs are
    /// subject to the following normalization and restrictions:
    ///
    /// 1. IDs must be valid Unicode strings.
    /// 2. IDs are limited to a maximum length of 64 characters.
    /// 3. IDs will be normalized according to Unicode Normalization Form C
    /// (<http://www.unicode.org/reports/tr15/>).
    /// 4. IDs may not contain any of the following ASCII characters: '/', ':',
    /// '\\', '?', or '#'.
    #[prost(string, tag = "4")]
    pub delivery_vehicle_id: ::prost::alloc::string::String,
    /// Required. The DeliveryVehicle entity to create.
    /// When creating a DeliveryVehicle, the following fields may optionally be
    /// set:
    ///   * last_location
    ///   * attributes
    /// All other DeliveryVehicle fields are ignored.
    #[prost(message, optional, tag = "5")]
    pub delivery_vehicle: ::core::option::Option<DeliveryVehicle>,
}
/// GetDeliveryVehicle request message.
/// Next id: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeliveryVehicleRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/deliveryVehicles/{delivery_vehicle}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// ListDeliveryVehicles request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryVehiclesRequest {
    /// Optional.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Vehicles to return. The service may return
    /// fewer than this value. If unspecified, the server will decide the number of
    /// results to return.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListDeliveryVehicles`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDeliveryVehicles`
    /// must match the call that provided the page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter query to apply when listing delivery vehicles.
    /// See <http://aip.dev/160> for examples of filter syntax.
    /// If no value is specified or filter is an empty string, all delivery
    /// vehicles will be returned.
    #[prost(string, tag = "6")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. A filter to limit the search area to a rectangle defined by the
    /// northeast and southwest corner points.
    ///
    /// When defined, only vehicles located within the area range will be returned.
    #[prost(message, optional, tag = "7")]
    pub viewport: ::core::option::Option<super::super::super::super::google::geo::r#type::Viewport>,
}
/// ListDeliveryVehicles response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryVehiclesResponse {
    /// The list of delivery vehicles that meet the requested filtering criteria.
    #[prost(message, repeated, tag = "1")]
    pub delivery_vehicles: ::prost::alloc::vec::Vec<DeliveryVehicle>,
    /// Pass this token in the ListDeliveryVehiclesRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of delivery vehicles matching request criteria across all
    /// pages.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
/// UpdateDeliveryVehicle request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeliveryVehicleRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. The DeliveryVehicle entity update to apply.
    /// The name of the DeliveryVehicle cannot be updated.
    #[prost(message, optional, tag = "3")]
    pub delivery_vehicle: ::core::option::Option<DeliveryVehicle>,
    /// Required. A field mask indicating which fields of the DeliveryVehicle to
    /// update. The update_mask must contain at least one field.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// CreateTask request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (e.g. `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Task ID must be unique for each provider. IDs are subject to
    /// the following normalization and restrictions:
    ///
    /// 1. IDs must be valid Unicode strings.
    /// 2. IDs are limited to a maximum length of 64 characters.
    /// 3. IDs will be normalized according to Unicode Normalization Form C
    /// (<http://www.unicode.org/reports/tr15/>).
    /// 4. IDs may not contain any of the following ASCII characters: '/', ':',
    /// '\\', '?', or '#'.
    #[prost(string, tag = "5")]
    pub task_id: ::prost::alloc::string::String,
    /// Required. Task entity to create.
    /// When creating a Task, the following fields are required:
    ///
    ///   * type
    ///   * state (must be set to 'OPEN' or request will fail)
    ///   * tracking_id (must not be set for UNAVAILABLE or SCHEDULED_STOP tasks
    ///       but required for all other task types.)
    ///   * planned_location (optional for UNAVAILABLE tasks)
    ///   * task_duration
    ///
    /// All other Task fields are ignored.
    #[prost(message, optional, tag = "4")]
    pub task: ::core::option::Option<Task>,
}
/// GetTask request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. Must be in the format `providers/{provider}/tasks/{task}`.
    /// The provider must be the Project ID (e.g. `sample-cloud-project`) of the
    /// Google Cloud Project of which the service account making this call is a
    /// member.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// SearchTasks request message containing the tracking_id used to search Tasks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTasksRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identifier for the related set of tasks that are being requested.
    #[prost(string, tag = "4")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Optional. The maximum number of Tasks to return. The service may return
    /// fewer than this value. If unspecified, the server will decide the number of
    /// results to return.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `SearchTasks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `SearchTasks` must match
    /// the call that provided the page token.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
}
/// SearchTasks response containing list of Tasks which met the search criteria
/// in the SearchTasksRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTasksResponse {
    /// The list of tasks for the requested tracking_id.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// Pass this token in the SearchTasksRequest to continue to
    /// list results. If all results have been returned, this field is an empty
    /// string or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// UpdateTask request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. The Task associated with the update.
    /// The following fields are maintained by the Fleet Engine. Do not update
    /// them using Task.update.
    ///   * last_location
    ///   * last_location_snappable
    ///   * name
    ///   * remaining_vehicle_journey_segments
    ///   * task_outcome_location_source
    ///
    /// The task_outcome cannot be changed once it has been set.
    ///
    /// If the Task has been assigned a delivery vehicle, do not set the Task state
    /// to CLOSED using Task.update. Instead, remove the VehicleStop containing
    /// the Task from the delivery vehicle, which automatically sets the Task
    /// state to CLOSED.
    #[prost(message, optional, tag = "3")]
    pub task: ::core::option::Option<Task>,
    /// Required. The field mask indicating which fields in Task to
    /// update. The update_mask must contain at least one field.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// ListTasks request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    /// Optional. The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::v1::RequestHeader>,
    /// Required. Must be in the format `providers/{provider}`.
    /// The provider must be the Project ID (for example, `sample-cloud-project`)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Tasks to return. The service may return
    /// fewer than this value. If unspecified, the server will decide the number of
    /// results to return.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListTasks` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTasks` must match
    /// the call that provided the page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter query to apply when listing tasks.
    /// See <http://aip.dev/160> for examples of filter syntax.
    /// If no value is specified or filter is an empty string, all tasks will be
    /// returned.
    #[prost(string, tag = "6")]
    pub filter: ::prost::alloc::string::String,
}
/// ListTasks response containing list of Tasks which met the filter criteria
/// in the ListTasksRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    /// The list of tasks that meet the requested filtering criteria.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// Pass this token in the ListTasksRequest to continue to list results.
    /// If all results have been returned, this field is an empty string or not
    /// present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of tasks matching request criteria across all pages.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
#[doc = r" Generated client implementations."]
pub mod delivery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Last Mile Delivery service."]
    #[derive(Debug, Clone)]
    pub struct DeliveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DeliveryServiceClient<T>
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
        ) -> DeliveryServiceClient<InterceptedService<T, F>>
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
            DeliveryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a DeliveryVehicle in the Fleet Engine and returns the new"]
        #[doc = " DeliveryVehicle."]
        pub async fn create_delivery_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeliveryVehicleRequest>,
        ) -> Result<tonic::Response<super::DeliveryVehicle>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/CreateDeliveryVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetDeliveryVehicle returns a DeliveryVehicle from the Fleet Engine."]
        pub async fn get_delivery_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeliveryVehicleRequest>,
        ) -> Result<tonic::Response<super::DeliveryVehicle>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/GetDeliveryVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpdateDeliveryVehicle writes updated DeliveryVehicle data to the Fleet"]
        #[doc = " Engine and assigns Tasks to the DeliveryVehicle. The name of the"]
        #[doc = " DeliveryVehicle cannot be updated. remaining_vehicle_journey_segments can"]
        #[doc = " be updated, but must contain all the VehicleJourneySegments currently on"]
        #[doc = " the DeliveryVehicle. The task_id's are retrieved from"]
        #[doc = " remaining_vehicle_journey_segments and their corresponding Tasks are"]
        #[doc = " assigned to the DeliveryVehicle if they have not yet been assigned to the"]
        #[doc = " vehicle."]
        pub async fn update_delivery_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeliveryVehicleRequest>,
        ) -> Result<tonic::Response<super::DeliveryVehicle>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/UpdateDeliveryVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Task in the Fleet Engine and returns the new Task."]
        pub async fn create_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/CreateTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a single Task."]
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/GetTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get all tasks with a specific tracking_id."]
        pub async fn search_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTasksRequest>,
        ) -> Result<tonic::Response<super::SearchTasksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/SearchTasks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates Task data."]
        pub async fn update_task(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/UpdateTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets all tasks that meet specified filtering criteria."]
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> Result<tonic::Response<super::ListTasksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/ListTasks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets all delivery vehicles that meet specified filtering criteria."]
        pub async fn list_delivery_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeliveryVehiclesRequest>,
        ) -> Result<tonic::Response<super::ListDeliveryVehiclesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.delivery.v1.DeliveryService/ListDeliveryVehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
