initSidebarItems({"enum":[["ContentType","Asset content type."]],"mod":[["analyze_iam_policy_response","Nested message and enum types in `AnalyzeIamPolicyResponse`."],["analyze_move_request","Nested message and enum types in `AnalyzeMoveRequest`."],["asset","Nested message and enum types in `Asset`."],["asset_service_client","Generated client implementations."],["condition_evaluation","Nested message and enum types in `ConditionEvaluation`."],["feed_output_config","Nested message and enum types in `FeedOutputConfig`."],["gcs_destination","Nested message and enum types in `GcsDestination`."],["iam_policy_analysis_output_config","Nested message and enum types in `IamPolicyAnalysisOutputConfig`."],["iam_policy_analysis_query","Nested message and enum types in `IamPolicyAnalysisQuery`."],["iam_policy_analysis_result","Nested message and enum types in `IamPolicyAnalysisResult`."],["iam_policy_search_result","Nested message and enum types in `IamPolicySearchResult`."],["move_analysis","Nested message and enum types in `MoveAnalysis`."],["output_config","Nested message and enum types in `OutputConfig`."],["output_result","Nested message and enum types in `OutputResult`."],["partition_spec","Nested message and enum types in `PartitionSpec`."],["temporal_asset","Nested message and enum types in `TemporalAsset`."]],"struct":[["AnalyzeIamPolicyLongrunningMetadata","Represents the metadata of the longrunning operation for the AnalyzeIamPolicyLongrunning rpc."],["AnalyzeIamPolicyLongrunningRequest","A request message for [AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning]."],["AnalyzeIamPolicyLongrunningResponse","A response message for [AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning]."],["AnalyzeIamPolicyRequest","A request message for [AssetService.AnalyzeIamPolicy][google.cloud.asset.v1.AssetService.AnalyzeIamPolicy]."],["AnalyzeIamPolicyResponse","A response message for [AssetService.AnalyzeIamPolicy][google.cloud.asset.v1.AssetService.AnalyzeIamPolicy]."],["AnalyzeMoveRequest","The request message for performing resource move analysis."],["AnalyzeMoveResponse","The response message for resource move analysis."],["Asset","An asset in Google Cloud. An asset can be any resource in the Google Cloud resource hierarchy, a resource outside the Google Cloud resource hierarchy (such as Google Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy), or a relationship (e.g. an INSTANCE_TO_INSTANCEGROUP relationship). See Supported asset types for more information."],["AttachedResource","Attached resource representation, which is defined by the corresponding service provider. It represents an attached resource’s payload."],["BatchGetAssetsHistoryRequest","Batch get assets history request."],["BatchGetAssetsHistoryResponse","Batch get assets history response."],["BigQueryDestination","A BigQuery destination for exporting assets to."],["ConditionEvaluation","The Condition evaluation."],["CreateFeedRequest","Create asset feed request."],["DeleteFeedRequest",""],["ExportAssetsRequest","Export asset request."],["ExportAssetsResponse","The export asset response. This message is returned by the [google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation] method in the returned [google.longrunning.Operation.response][google.longrunning.Operation.response] field."],["Feed","An asset feed used to export asset updates to a destinations. An asset feed filter controls what updates are exported. The asset feed must be created within a project, organization, or folder. Supported destinations are: Pub/Sub topics."],["FeedOutputConfig","Output configuration for asset feed destination."],["GcsDestination","A Cloud Storage location."],["GcsOutputResult","A Cloud Storage output result."],["GetFeedRequest","Get asset feed request."],["IamPolicyAnalysisOutputConfig","Output configuration for export IAM policy analysis destination."],["IamPolicyAnalysisQuery","IAM policy analysis query message."],["IamPolicyAnalysisResult","IAM Policy analysis result, consisting of one IAM policy binding and derived access control lists."],["IamPolicyAnalysisState","Represents the detailed state of an entity under analysis, such as a resource, an identity or an access."],["IamPolicySearchResult","A result of IAM Policy search, containing information of an IAM policy."],["ListAssetsRequest","ListAssets request."],["ListAssetsResponse","ListAssets response."],["ListFeedsRequest","List asset feeds request."],["ListFeedsResponse",""],["MoveAnalysis","A message to group the analysis information."],["MoveAnalysisResult","An analysis result including blockers and warnings."],["MoveImpact","A message to group impacts of moving the target resource."],["OutputConfig","Output configuration for export assets destination."],["OutputResult","Output result of export assets."],["PartitionSpec","Specifications of BigQuery partitioned table as export destination."],["PubsubDestination","A Pub/Sub destination."],["RelatedAsset","An asset identify in Google Cloud which contains its name, type and ancestors. An asset can be any resource in the Google Cloud resource hierarchy, a resource outside the Google Cloud resource hierarchy (such as Google Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy). See Supported asset types for more information."],["RelatedAssets","The detailed related assets with the `relationship_type`."],["RelatedResource","The detailed related resource."],["RelatedResources","The related resources of the primary resource."],["RelationshipAttributes","The relationship attributes which include  `type`, `source_resource_type`, `target_resource_type` and `action`."],["Resource","A representation of a Google Cloud resource."],["ResourceSearchResult","A result of Resource Search, containing information of a cloud resource."],["SearchAllIamPoliciesRequest","Search all IAM policies request."],["SearchAllIamPoliciesResponse","Search all IAM policies response."],["SearchAllResourcesRequest","Search all resources request."],["SearchAllResourcesResponse","Search all resources response."],["TemporalAsset","An asset in Google Cloud and its temporal metadata, including the time window when it was observed and its status during that window."],["TimeWindow","A time window specified by its `start_time` and `end_time`."],["UpdateFeedRequest","Update asset feed request."],["VersionedResource","Resource representation as defined by the corresponding service providing the resource for a given API version."]]});