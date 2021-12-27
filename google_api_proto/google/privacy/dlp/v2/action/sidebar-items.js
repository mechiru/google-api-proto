initSidebarItems({"enum":[["Action",""]],"struct":[["JobNotificationEmails","Enable email notification to project owners and editors on jobs’s completion/failure."],["PublishFindingsToCloudDataCatalog","Publish findings of a DlpJob to Data Catalog. Labels summarizing the results of the DlpJob will be applied to the entry for the resource scanned in Data Catalog. Any labels previously written by another DlpJob will be deleted. InfoType naming patterns are strictly enforced when using this feature. Note that the findings will be persisted in Data Catalog storage and are governed by Data Catalog service-specific policy, see https://cloud.google.com/terms/service-terms Only a single instance of this action can be specified and only allowed if all resources being scanned are BigQuery tables. Compatible with: Inspect"],["PublishSummaryToCscc","Publish the result summary of a DlpJob to the Cloud Security Command Center (CSCC Alpha). This action is only available for projects which are parts of an organization and whitelisted for the alpha Cloud Security Command Center. The action will publish count of finding instances and their info types. The summary of findings will be persisted in CSCC and are governed by CSCC service-specific policy, see https://cloud.google.com/terms/service-terms Only a single instance of this action can be specified. Compatible with: Inspect"],["PublishToPubSub","Publish a message into given Pub/Sub topic when DlpJob has completed. The message contains a single field, `DlpJobName`, which is equal to the finished job’s [`DlpJob.name`](https://cloud.google.com/dlp/docs/reference/rest/v2/projects.dlpJobs#DlpJob). Compatible with: Inspect, Risk"],["PublishToStackdriver","Enable Stackdriver metric dlp.googleapis.com/finding_count. This will publish a metric to stack driver on each infotype requested and how many findings were found for it. CustomDetectors will be bucketed as ‘Custom’ under the Stackdriver label ‘info_type’."],["SaveFindings","If set, the detailed findings will be persisted to the specified OutputStorageConfig. Only a single instance of this action can be specified. Compatible with: Inspect, Risk"]]});