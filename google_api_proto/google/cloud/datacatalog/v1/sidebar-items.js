initSidebarItems({"enum":[["EntryType","The enum field that lists all the types of entry resources in Data Catalog. For example, a BigQuery table entry has the `TABLE` type."],["IntegratedSystem","This enum lists all the systems that Data Catalog integrates with."],["SearchResultType","The resource types that can be returned in search results."],["TableSourceType","Table source type."]],"mod":[["big_query_connection_spec","Nested message and enum types in `BigQueryConnectionSpec`."],["big_query_table_spec","Nested message and enum types in `BigQueryTableSpec`."],["cloud_sql_big_query_connection_spec","Nested message and enum types in `CloudSqlBigQueryConnectionSpec`."],["contacts","Nested message and enum types in `Contacts`."],["data_catalog_client","Generated client implementations."],["data_source","Nested message and enum types in `DataSource`."],["database_table_spec","Nested message and enum types in `DatabaseTableSpec`."],["entry","Nested message and enum types in `Entry`."],["export_taxonomies_request","Nested message and enum types in `ExportTaxonomiesRequest`."],["field_type","Nested message and enum types in `FieldType`."],["import_taxonomies_request","Nested message and enum types in `ImportTaxonomiesRequest`."],["lookup_entry_request","Nested message and enum types in `LookupEntryRequest`."],["policy_tag_manager_client","Generated client implementations."],["policy_tag_manager_serialization_client","Generated client implementations."],["routine_spec","Nested message and enum types in `RoutineSpec`."],["search_catalog_request","Nested message and enum types in `SearchCatalogRequest`."],["search_catalog_result","Nested message and enum types in `SearchCatalogResult`."],["tag","Nested message and enum types in `Tag`."],["tag_field","Nested message and enum types in `TagField`."],["taxonomy","Nested message and enum types in `Taxonomy`."]],"struct":[["BigQueryConnectionSpec","Specification for the BigQuery connection."],["BigQueryDateShardedSpec","Specification for a group of BigQuery tables with the `\\[prefix\\]YYYYMMDD` name pattern."],["BigQueryRoutineSpec","Fields specific for BigQuery routines."],["BigQueryTableSpec","Describes a BigQuery table."],["BusinessContext","Business Context of the entry."],["CloudSqlBigQueryConnectionSpec","Specification for the BigQuery connection to a Cloud SQL instance."],["ColumnSchema","A column within a schema. Columns can be nested inside other columns."],["Contacts","Contact people for the entry."],["CreateEntryGroupRequest","Request message for [CreateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.CreateEntryGroup]."],["CreateEntryRequest","Request message for [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry]."],["CreatePolicyTagRequest","Request message for [CreatePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.CreatePolicyTag]."],["CreateTagRequest","Request message for [CreateTag][google.cloud.datacatalog.v1.DataCatalog.CreateTag]."],["CreateTagTemplateFieldRequest","Request message for [CreateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplateField]."],["CreateTagTemplateRequest","Request message for [CreateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplate]."],["CreateTaxonomyRequest","Request message for [CreateTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.CreateTaxonomy]."],["CrossRegionalSource","Cross-regional source used to import an existing taxonomy into a different region."],["DataSource","Physical location of an entry."],["DataSourceConnectionSpec","Specification that applies to a data source connection. Valid only for entries with the `DATA_SOURCE_CONNECTION` type."],["DatabaseTableSpec","Specification that applies to a table resource. Valid only for entries with the `TABLE` type."],["DeleteEntryGroupRequest","Request message for [DeleteEntryGroup][google.cloud.datacatalog.v1.DataCatalog.DeleteEntryGroup]."],["DeleteEntryRequest","Request message for [DeleteEntry][google.cloud.datacatalog.v1.DataCatalog.DeleteEntry]."],["DeletePolicyTagRequest","Request message for [DeletePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.DeletePolicyTag]."],["DeleteTagRequest","Request message for [DeleteTag][google.cloud.datacatalog.v1.DataCatalog.DeleteTag]."],["DeleteTagTemplateFieldRequest","Request message for [DeleteTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplateField]."],["DeleteTagTemplateRequest","Request message for [DeleteTagTemplate][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplate]."],["DeleteTaxonomyRequest","Request message for [DeleteTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.DeleteTaxonomy]."],["Entry","Entry metadata. A Data Catalog entry represents another resource in Google Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic) or outside of it. You can use the `linked_resource` field in the entry resource to refer to the original resource ID of the source system."],["EntryGroup","Entry group metadata."],["EntryOverview","Entry overview fields for rich text descriptions of entries."],["ExportTaxonomiesRequest","Request message for [ExportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ExportTaxonomies]."],["ExportTaxonomiesResponse","Response message for [ExportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ExportTaxonomies]."],["FieldType",""],["GcsFileSpec","Specification of a single file in Cloud Storage."],["GcsFilesetSpec","Describes a Cloud Storage fileset entry."],["GetEntryGroupRequest","Request message for [GetEntryGroup][google.cloud.datacatalog.v1.DataCatalog.GetEntryGroup]."],["GetEntryRequest","Request message for [GetEntry][google.cloud.datacatalog.v1.DataCatalog.GetEntry]."],["GetPolicyTagRequest","Request message for [GetPolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.GetPolicyTag]."],["GetTagTemplateRequest","Request message for [GetTagTemplate][google.cloud.datacatalog.v1.DataCatalog.GetTagTemplate]."],["GetTaxonomyRequest","Request message for [GetTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.GetTaxonomy]."],["ImportTaxonomiesRequest","Request message for [ImportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ImportTaxonomies]."],["ImportTaxonomiesResponse","Response message for [ImportTaxonomies][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ImportTaxonomies]."],["InlineSource","Inline source containing taxonomies to import."],["ListEntriesRequest","Request message for [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries]."],["ListEntriesResponse","Response message for [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries]."],["ListEntryGroupsRequest","Request message for [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups]."],["ListEntryGroupsResponse","Response message for [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups]."],["ListPolicyTagsRequest","Request message for [ListPolicyTags][google.cloud.datacatalog.v1.PolicyTagManager.ListPolicyTags]."],["ListPolicyTagsResponse","Response message for [ListPolicyTags][google.cloud.datacatalog.v1.PolicyTagManager.ListPolicyTags]."],["ListTagsRequest","Request message for [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags]."],["ListTagsResponse","Response message for [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags]."],["ListTaxonomiesRequest","Request message for [ListTaxonomies][google.cloud.datacatalog.v1.PolicyTagManager.ListTaxonomies]."],["ListTaxonomiesResponse","Response message for [ListTaxonomies][google.cloud.datacatalog.v1.PolicyTagManager.ListTaxonomies]."],["LookupEntryRequest","Request message for [LookupEntry][google.cloud.datacatalog.v1.DataCatalog.LookupEntry]."],["ModifyEntryContactsRequest","Request message for [ModifyEntryContacts][google.cloud.datacatalog.v1.DataCatalog.ModifyEntryContacts]."],["ModifyEntryOverviewRequest","Request message for [ModifyEntryOverview][google.cloud.datacatalog.v1.DataCatalog.ModifyEntryOverview]."],["PersonalDetails","Entry metadata relevant only to the user and private to them."],["PolicyTag","Denotes one policy tag in a taxonomy, for example, SSN."],["RenameTagTemplateFieldEnumValueRequest","Request message for [RenameTagTemplateFieldEnumValue][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateFieldEnumValue]."],["RenameTagTemplateFieldRequest","Request message for [RenameTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateField]."],["ReplaceTaxonomyRequest","Request message for [ReplaceTaxonomy][google.cloud.datacatalog.v1.PolicyTagManagerSerialization.ReplaceTaxonomy]."],["RoutineSpec","Specification that applies to a routine. Valid only for entries with the `ROUTINE` type."],["Schema","Represents a schema, for example, a BigQuery, GoogleSQL, or Avro schema."],["SearchCatalogRequest","Request message for [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog]."],["SearchCatalogResponse","Response message for [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog]."],["SearchCatalogResult","Result in the response to a search request."],["SerializedPolicyTag","A nested protocol buffer that represents a policy tag and all its descendants."],["SerializedTaxonomy","A nested protocol buffer that represents a taxonomy and the hierarchy of its policy tags. Used for taxonomy replacement, import, and export."],["StarEntryRequest","Request message for [StarEntry][google.cloud.datacatalog.v1.DataCatalog.StarEntry]."],["StarEntryResponse","Response message for [StarEntry][google.cloud.datacatalog.v1.DataCatalog.StarEntry]. Empty for now"],["SystemTimestamps","Timestamps associated with this resource in a particular system."],["TableSpec","Normal BigQuery table specification."],["Tag","Tags contain custom metadata and are attached to Data Catalog resources. Tags conform with the specification of their tag template."],["TagField","Contains the value and additional information on a field within a [Tag][google.cloud.datacatalog.v1.Tag]."],["TagTemplate","A tag template defines a tag that can have one or more typed fields."],["TagTemplateField","The template for an individual field within a tag template."],["Taxonomy","A taxonomy is a collection of hierarchical policy tags that classify data along a common axis."],["UnstarEntryRequest","Request message for [UnstarEntry][google.cloud.datacatalog.v1.DataCatalog.UnstarEntry]."],["UnstarEntryResponse","Response message for [UnstarEntry][google.cloud.datacatalog.v1.DataCatalog.UnstarEntry]. Empty for now"],["UpdateEntryGroupRequest","Request message for [UpdateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.UpdateEntryGroup]."],["UpdateEntryRequest","Request message for [UpdateEntry][google.cloud.datacatalog.v1.DataCatalog.UpdateEntry]."],["UpdatePolicyTagRequest","Request message for [UpdatePolicyTag][google.cloud.datacatalog.v1.PolicyTagManager.UpdatePolicyTag]."],["UpdateTagRequest","Request message for [UpdateTag][google.cloud.datacatalog.v1.DataCatalog.UpdateTag]."],["UpdateTagTemplateFieldRequest","Request message for [UpdateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplateField]."],["UpdateTagTemplateRequest","Request message for [UpdateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplate]."],["UpdateTaxonomyRequest","Request message for [UpdateTaxonomy][google.cloud.datacatalog.v1.PolicyTagManager.UpdateTaxonomy]."],["UsageSignal","The set of all usage signals that Data Catalog stores."],["UsageStats","Detailed statistics on the entry’s usage."],["ViewSpec","Table view specification."]]});