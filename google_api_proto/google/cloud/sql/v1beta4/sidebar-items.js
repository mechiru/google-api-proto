initSidebarItems({"enum":[["SqlAvailabilityType","The availability type of the given Cloud SQL instance."],["SqlBackendType",""],["SqlBackupKind","Defines the supported backup kinds"],["SqlBackupRunStatus","The status of a backup run."],["SqlBackupRunType",""],["SqlDataDiskType","The type of disk that is used for a v2 instance to use."],["SqlDatabaseVersion","The database engine type and version."],["SqlFileType",""],["SqlFlagType",""],["SqlInstanceType",""],["SqlIpAddressType",""],["SqlPricingPlan","The pricing plan for this instance."],["SqlReplicationType",""],["SqlSuspensionReason","The suspension reason of the database instance if the state is SUSPENDED."],["SqlUpdateTrack",""]],"mod":[["api_warning","Nested message and enum types in `ApiWarning`."],["backup_retention_settings","Nested message and enum types in `BackupRetentionSettings`."],["database","Nested message and enum types in `Database`."],["database_instance","Nested message and enum types in `DatabaseInstance`."],["export_context","Nested message and enum types in `ExportContext`."],["import_context","Nested message and enum types in `ImportContext`."],["operation","Nested message and enum types in `Operation`."],["password_validation_policy","Nested message and enum types in `PasswordValidationPolicy`."],["settings","Nested message and enum types in `Settings`."],["sql_backup_runs_service_client","Generated client implementations."],["sql_connect_service_client","Generated client implementations."],["sql_databases_service_client","Generated client implementations."],["sql_external_sync_setting_error","Nested message and enum types in `SqlExternalSyncSettingError`."],["sql_flags_service_client","Generated client implementations."],["sql_instances_reschedule_maintenance_request_body","Nested message and enum types in `SqlInstancesRescheduleMaintenanceRequestBody`."],["sql_instances_service_client","Generated client implementations."],["sql_instances_start_external_sync_request","Nested message and enum types in `SqlInstancesStartExternalSyncRequest`."],["sql_instances_verify_external_sync_settings_request","Nested message and enum types in `SqlInstancesVerifyExternalSyncSettingsRequest`."],["sql_operations_service_client","Generated client implementations."],["sql_ssl_certs_service_client","Generated client implementations."],["sql_tiers_service_client","Generated client implementations."],["sql_users_service_client","Generated client implementations."],["user","Nested message and enum types in `User`."]],"struct":[["AclEntry","An entry for an Access Control list."],["ApiWarning","An Admin API warning message."],["BackupConfiguration","Database instance backup configuration."],["BackupContext","Backup context."],["BackupRetentionSettings","We currently only support backup retention by specifying the number of backups we will retain."],["BackupRun","A BackupRun resource."],["BackupRunsListResponse","Backup run list results."],["BinLogCoordinates","Binary log coordinates."],["CloneContext","Database instance clone context."],["ConnectSettings","Connect settings retrieval response."],["Database","Represents a SQL database on the Cloud SQL instance."],["DatabaseFlags","Database flags for Cloud SQL instances."],["DatabaseInstance","A Cloud SQL instance resource."],["DatabasesListResponse","Database list response."],["DemoteMasterConfiguration","Read-replica configuration for connecting to the on-premises primary instance."],["DemoteMasterContext","Database instance demote primary instance context."],["DemoteMasterMySqlReplicaConfiguration","Read-replica configuration specific to MySQL databases."],["DenyMaintenancePeriod","Deny Maintenance Periods. This specifies a date range during when all CSA rollout will be denied."],["DiskEncryptionConfiguration","Disk encryption configuration for an instance."],["DiskEncryptionStatus","Disk encryption status for an instance."],["ExportContext","Database instance export context."],["FailoverContext","Database instance failover context."],["Flag","A flag resource."],["FlagsListResponse","Flags list response."],["GenerateEphemeralCertRequest","Ephemeral certificate creation request."],["GenerateEphemeralCertResponse","Ephemeral certificate creation request."],["GetConnectSettingsRequest","Connect settings retrieval request."],["ImportContext","Database instance import context."],["InsightsConfig","Insights configuration. This specifies when Cloud SQL Insights feature is enabled and optional configuration."],["InstanceReference","Reference to another Cloud SQL instance."],["InstancesCloneRequest","Database instance clone request."],["InstancesDemoteMasterRequest","Database demote primary instance request."],["InstancesExportRequest","Database instance export request."],["InstancesFailoverRequest","Instance failover request."],["InstancesImportRequest","Database instance import request."],["InstancesListResponse","Database instances list response."],["InstancesListServerCasResponse","Instances ListServerCas response."],["InstancesRestoreBackupRequest","Database instance restore backup request."],["InstancesRotateServerCaRequest","Rotate Server CA request."],["InstancesTruncateLogRequest","Instance truncate log request."],["IpConfiguration","IP Management configuration."],["IpMapping","Database instance IP Mapping."],["LocationPreference","Preferred location. This specifies where a Cloud SQL instance is located. Note that if the preferred location is not available, the instance will be located as close as possible within the region. Only one location may be specified."],["MaintenanceWindow","Maintenance window. This specifies when a Cloud SQL instance is restarted for system maintenance purposes."],["MySqlReplicaConfiguration","Read-replica configuration specific to MySQL databases."],["MySqlSyncConfig","MySQL-specific external server sync settings."],["OnPremisesConfiguration","On-premises instance configuration."],["Operation","An Operation resource. For successful operations that return an Operation resource, only the fields relevant to the operation are populated in the resource."],["OperationError","Database instance operation error."],["OperationErrors","Database instance operation errors list wrapper."],["OperationsListResponse","Operations list response."],["PasswordStatus","Read-only password status."],["PasswordValidationPolicy","Database instance local user password validation policy"],["ReplicaConfiguration","Read-replica configuration for connecting to the primary instance."],["RestoreBackupContext","Database instance restore from backup context. Backup context contains source instance id and project id."],["RotateServerCaContext","Instance rotate server CA context."],["Settings","Database instance settings."],["SqlActiveDirectoryConfig","Active Directory configuration, relevant only for Cloud SQL for SQL Server."],["SqlBackupRunsDeleteRequest",""],["SqlBackupRunsGetRequest",""],["SqlBackupRunsInsertRequest",""],["SqlBackupRunsListRequest",""],["SqlDatabasesDeleteRequest",""],["SqlDatabasesGetRequest",""],["SqlDatabasesInsertRequest",""],["SqlDatabasesListRequest",""],["SqlDatabasesUpdateRequest",""],["SqlExternalSyncSettingError","External primary instance migration setting error/warning."],["SqlFlagsListRequest",""],["SqlInstancesAddServerCaRequest",""],["SqlInstancesCloneRequest",""],["SqlInstancesCreateEphemeralCertRequest",""],["SqlInstancesDeleteRequest",""],["SqlInstancesDemoteMasterRequest",""],["SqlInstancesExportRequest",""],["SqlInstancesFailoverRequest",""],["SqlInstancesGetRequest",""],["SqlInstancesImportRequest",""],["SqlInstancesInsertRequest",""],["SqlInstancesListRequest",""],["SqlInstancesListServerCasRequest",""],["SqlInstancesPatchRequest",""],["SqlInstancesPromoteReplicaRequest",""],["SqlInstancesRescheduleMaintenanceRequest",""],["SqlInstancesRescheduleMaintenanceRequestBody","Reschedule options for maintenance windows."],["SqlInstancesResetSslConfigRequest",""],["SqlInstancesRestartRequest",""],["SqlInstancesRestoreBackupRequest",""],["SqlInstancesRotateServerCaRequest",""],["SqlInstancesStartExternalSyncRequest",""],["SqlInstancesStartReplicaRequest",""],["SqlInstancesStopReplicaRequest",""],["SqlInstancesTruncateLogRequest",""],["SqlInstancesUpdateRequest",""],["SqlInstancesVerifyExternalSyncSettingsRequest",""],["SqlInstancesVerifyExternalSyncSettingsResponse","Instance verify external sync settings response."],["SqlOperationsGetRequest",""],["SqlOperationsListRequest",""],["SqlServerAuditConfig","SQL Server specific audit configuration."],["SqlServerDatabaseDetails","Represents a Sql Server database on the Cloud SQL instance."],["SqlServerUserDetails","Represents a Sql Server user on the Cloud SQL instance."],["SqlSslCertsDeleteRequest",""],["SqlSslCertsGetRequest",""],["SqlSslCertsInsertRequest",""],["SqlSslCertsListRequest",""],["SqlTiersListRequest",""],["SqlUsersDeleteRequest",""],["SqlUsersInsertRequest",""],["SqlUsersListRequest",""],["SqlUsersUpdateRequest",""],["SslCert","SslCerts Resource"],["SslCertDetail","SslCertDetail."],["SslCertsCreateEphemeralRequest","SslCerts create ephemeral certificate request."],["SslCertsInsertRequest","SslCerts insert request."],["SslCertsInsertResponse","SslCert insert response."],["SslCertsListResponse","SslCerts list response."],["SyncFlags","Initial sync flags for certain Cloud SQL APIs. Currently used for the MySQL external server initial dump."],["Tier","A Google Cloud SQL service tier resource."],["TiersListResponse","Tiers list response."],["TruncateLogContext","Database Instance truncate log context."],["User","A Cloud SQL user resource."],["UserPasswordValidationPolicy","User level password validation policy."],["UsersListResponse","User list response."]]});