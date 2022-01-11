initSidebarItems({"mod":[["admin_service_client","Generated client implementations."],["cursor_service_client","Generated client implementations."],["partition_assignment_request","Nested message and enum types in `PartitionAssignmentRequest`."],["partition_assignment_service_client","Generated client implementations."],["publish_request","Nested message and enum types in `PublishRequest`."],["publish_response","Nested message and enum types in `PublishResponse`."],["publisher_service_client","Generated client implementations."],["seek_request","Nested message and enum types in `SeekRequest`."],["seek_subscription_request","Nested message and enum types in `SeekSubscriptionRequest`."],["streaming_commit_cursor_request","Nested message and enum types in `StreamingCommitCursorRequest`."],["streaming_commit_cursor_response","Nested message and enum types in `StreamingCommitCursorResponse`."],["subscribe_request","Nested message and enum types in `SubscribeRequest`."],["subscribe_response","Nested message and enum types in `SubscribeResponse`."],["subscriber_service_client","Generated client implementations."],["subscription","Nested message and enum types in `Subscription`."],["time_target","Nested message and enum types in `TimeTarget`."],["topic","Nested message and enum types in `Topic`."],["topic_stats_service_client","Generated client implementations."]],"struct":[["AttributeValues","The values associated with a key of an attribute."],["CommitCursorRequest","Request for CommitCursor."],["CommitCursorResponse","Response for CommitCursor."],["ComputeHeadCursorRequest","Compute the current head cursor for a partition."],["ComputeHeadCursorResponse","Response containing the head cursor for the requested topic and partition."],["ComputeMessageStatsRequest","Compute statistics about a range of messages in a given topic and partition."],["ComputeMessageStatsResponse","Response containing stats for messages in the requested topic and partition."],["ComputeTimeCursorRequest","Compute the corresponding cursor for a publish or event time in a topic partition."],["ComputeTimeCursorResponse","Response containing the cursor corresponding to a publish or event time in a topic partition."],["CreateReservationRequest","Request for CreateReservation."],["CreateSubscriptionRequest","Request for CreateSubscription."],["CreateTopicRequest","Request for CreateTopic."],["Cursor","A cursor that describes the position of a message within a topic partition."],["DeleteReservationRequest","Request for DeleteReservation."],["DeleteSubscriptionRequest","Request for DeleteSubscription."],["DeleteTopicRequest","Request for DeleteTopic."],["FlowControlRequest","Request to grant tokens to the server, requesting delivery of messages when they become available."],["GetReservationRequest","Request for GetReservation."],["GetSubscriptionRequest","Request for GetSubscription."],["GetTopicPartitionsRequest","Request for GetTopicPartitions."],["GetTopicRequest","Request for GetTopic."],["InitialCommitCursorRequest","The first streaming request that must be sent on a newly-opened stream. The client must wait for the response before sending subsequent requests on the stream."],["InitialCommitCursorResponse","Response to an InitialCommitCursorRequest."],["InitialPartitionAssignmentRequest","The first request that must be sent on a newly-opened stream. The client must wait for the response before sending subsequent requests on the stream."],["InitialPublishRequest","The first request that must be sent on a newly-opened stream."],["InitialPublishResponse","Response to an InitialPublishRequest."],["InitialSubscribeRequest","The first request that must be sent on a newly-opened stream. The client must wait for the response before sending subsequent requests on the stream."],["InitialSubscribeResponse","Response to an InitialSubscribeRequest."],["ListPartitionCursorsRequest","Request for ListPartitionCursors."],["ListPartitionCursorsResponse","Response for ListPartitionCursors"],["ListReservationTopicsRequest","Request for ListReservationTopics."],["ListReservationTopicsResponse","Response for ListReservationTopics."],["ListReservationsRequest","Request for ListReservations."],["ListReservationsResponse","Response for ListReservations."],["ListSubscriptionsRequest","Request for ListSubscriptions."],["ListSubscriptionsResponse","Response for ListSubscriptions."],["ListTopicSubscriptionsRequest","Request for ListTopicSubscriptions."],["ListTopicSubscriptionsResponse","Response for ListTopicSubscriptions."],["ListTopicsRequest","Request for ListTopics."],["ListTopicsResponse","Response for ListTopics."],["MessagePublishRequest","Request to publish messages to the topic."],["MessagePublishResponse","Response to a MessagePublishRequest."],["MessageResponse","Response containing a list of messages. Upon delivering a MessageResponse to the client, the server:"],["OperationMetadata","Metadata for long running operations."],["PartitionAssignment","PartitionAssignments should not race with acknowledgements. There should be exactly one unacknowledged PartitionAssignment at a time. If not, the client must break the stream."],["PartitionAssignmentAck","Acknowledge receipt and handling of the previous assignment. If not sent within a short period after receiving the assignment, partitions may remain unassigned for a period of time until the client is known to be inactive, after which time the server will break the stream."],["PartitionAssignmentRequest","A request on the PartitionAssignment stream."],["PartitionCursor","A pair of a Cursor and the partition it is for."],["PubSubMessage","A message that is published by publishers and delivered to subscribers."],["PublishRequest","Request sent from the client to the server on a stream."],["PublishResponse","Response to a PublishRequest."],["Reservation","Metadata about a reservation resource."],["SeekRequest","Request to update the stream’s delivery cursor based on the given target. Resets the server available tokens to 0. SeekRequests past head result in stream breakage."],["SeekResponse","Response to a SeekRequest."],["SeekSubscriptionRequest","Request for SeekSubscription."],["SeekSubscriptionResponse","Response for SeekSubscription long running operation."],["SequencedCommitCursorRequest","Streaming request to update the committed cursor. Subsequent SequencedCommitCursorRequests override outstanding ones."],["SequencedCommitCursorResponse","Response to a SequencedCommitCursorRequest."],["SequencedMessage","A message that has been stored and sequenced by the Pub/Sub Lite system."],["StreamingCommitCursorRequest","A request sent from the client to the server on a stream."],["StreamingCommitCursorResponse","Response to a StreamingCommitCursorRequest."],["SubscribeRequest","A request sent from the client to the server on a stream."],["SubscribeResponse","Response to SubscribeRequest."],["Subscription","Metadata about a subscription resource."],["TimeTarget","A target publish or event time. Can be used for seeking to or retrieving the corresponding cursor."],["Topic","Metadata about a topic resource."],["TopicPartitions","Response for GetTopicPartitions."],["UpdateReservationRequest","Request for UpdateReservation."],["UpdateSubscriptionRequest","Request for UpdateSubscription."],["UpdateTopicRequest","Request for UpdateTopic."]]});