initSidebarItems({"enum":[["DeliveryVehicleLocationSensor","The sensor or methodology used to determine the location."],["DeliveryVehicleNavigationStatus","The vehicle’s navigation status."]],"mod":[["delivery_request_header","Nested message and enum types in `DeliveryRequestHeader`."],["delivery_service_client","Generated client implementations."],["task","Nested message and enum types in `Task`."],["vehicle_stop","Nested message and enum types in `VehicleStop`."]],"struct":[["CreateDeliveryVehicleRequest","The `CreateDeliveryVehicle` request message."],["CreateTaskRequest","The `CreateTask` request message."],["DeliveryRequestHeader","A RequestHeader contains fields common to all Delivery RPC requests."],["DeliveryVehicle","The `DeliveryVehicle` message. A delivery vehicle transports shipments from a depot to a delivery location, and from a pickup location to the depot. In some cases, delivery vehicles also transport shipments directly from the pickup location to the delivery location."],["DeliveryVehicleAttribute","Describes a vehicle attribute as a key-value pair. The “key:value” string length cannot exceed 256 characters."],["DeliveryVehicleLocation","The location, speed, and heading of a vehicle at a point in time."],["GetDeliveryVehicleRequest","The `GetDeliveryVehicle` request message. Next id: 4"],["GetTaskRequest","The `GetTask` request message."],["ListDeliveryVehiclesRequest","The `ListDeliveryVehicles` request message."],["ListDeliveryVehiclesResponse","The `ListDeliveryVehicles` response message."],["ListTasksRequest","The `ListTasks` request message."],["ListTasksResponse","The `ListTasks` response that contains the set of Tasks that meet the filter criteria in the `ListTasksRequest`."],["LocationInfo","A location with any additional identifiers."],["SearchTasksRequest","The `SearchTasks` request message that contains the `tracking_id`."],["SearchTasksResponse","The `SearchTasks` response. It contains the set of Tasks that meet the search criteria in the `SearchTasksRequest`."],["Task","A Task in the Delivery API represents a single action to track. In general, there’s a distinction between shipment-related Tasks, and break Tasks. A shipment can have multiple Tasks associated with it. For example, there could be one Task for the pickup, and one for the dropoff or transfer. And different Tasks for a given shipment can be handled by different vehicles. For example, one vehicle could handle the pickup, driving the shipment to the hub, while another vehicle drives the same shipment from the hub to the dropoff location."],["UpdateDeliveryVehicleRequest","The `UpdateDeliveryVehicle` request message."],["UpdateTaskRequest","The `UpdateTask` request message."],["VehicleJourneySegment","Represents a Vehicle’s travel segment - from its previous stop to the current stop. If it’s the first active stop, then it’s from the Vehicle’s current location to this stop."],["VehicleStop","Describes a point where a Vehicle stops to perform one or more Tasks."]]});