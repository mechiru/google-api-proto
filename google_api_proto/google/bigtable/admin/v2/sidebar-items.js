initSidebarItems({"enum":[["RestoreSourceType","Indicates the type of the restore source."],["StorageType","Storage media types for persisting Bigtable data."]],"mod":[["app_profile","Nested message and enum types in `AppProfile`."],["backup","Nested message and enum types in `Backup`."],["bigtable_instance_admin_client","Generated client implementations."],["bigtable_table_admin_client","Generated client implementations."],["cluster","Nested message and enum types in `Cluster`."],["create_table_request","Nested message and enum types in `CreateTableRequest`."],["drop_row_range_request","Nested message and enum types in `DropRowRangeRequest`."],["encryption_info","Nested message and enum types in `EncryptionInfo`."],["gc_rule","Nested message and enum types in `GcRule`."],["instance","Nested message and enum types in `Instance`."],["modify_column_families_request","Nested message and enum types in `ModifyColumnFamiliesRequest`."],["restore_info","Nested message and enum types in `RestoreInfo`."],["restore_table_metadata","Nested message and enum types in `RestoreTableMetadata`."],["restore_table_request","Nested message and enum types in `RestoreTableRequest`."],["snapshot","Nested message and enum types in `Snapshot`."],["table","Nested message and enum types in `Table`."]],"struct":[["AppProfile","A configuration object describing how Cloud Bigtable should treat traffic from a particular end user application."],["AutoscalingLimits","Limits for the number of nodes a Cluster can autoscale up/down to."],["AutoscalingTargets","The Autoscaling targets for a Cluster. These determine the recommended nodes."],["Backup","A backup of a Cloud Bigtable table."],["BackupInfo","Information about a backup."],["CheckConsistencyRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency][google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency]"],["CheckConsistencyResponse","Response message for [google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency][google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency]"],["Cluster","A resizable group of nodes in a particular cloud location, capable of serving all [Tables][google.bigtable.admin.v2.Table] in the parent [Instance][google.bigtable.admin.v2.Instance]."],["ColumnFamily","A set of columns within a table which share a common configuration."],["CreateAppProfileRequest","Request message for BigtableInstanceAdmin.CreateAppProfile."],["CreateBackupMetadata","Metadata type for the operation returned by [CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup]."],["CreateBackupRequest","The request for [CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup]."],["CreateClusterMetadata","The metadata for the Operation returned by CreateCluster."],["CreateClusterRequest","Request message for BigtableInstanceAdmin.CreateCluster."],["CreateInstanceMetadata","The metadata for the Operation returned by CreateInstance."],["CreateInstanceRequest","Request message for BigtableInstanceAdmin.CreateInstance."],["CreateTableFromSnapshotMetadata","The metadata for the Operation returned by CreateTableFromSnapshot."],["CreateTableFromSnapshotRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.CreateTableFromSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.CreateTableFromSnapshot]"],["CreateTableRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.CreateTable][google.bigtable.admin.v2.BigtableTableAdmin.CreateTable]"],["DeleteAppProfileRequest","Request message for BigtableInstanceAdmin.DeleteAppProfile."],["DeleteBackupRequest","The request for [DeleteBackup][google.bigtable.admin.v2.BigtableTableAdmin.DeleteBackup]."],["DeleteClusterRequest","Request message for BigtableInstanceAdmin.DeleteCluster."],["DeleteInstanceRequest","Request message for BigtableInstanceAdmin.DeleteInstance."],["DeleteSnapshotRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.DeleteSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.DeleteSnapshot]"],["DeleteTableRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.DeleteTable][google.bigtable.admin.v2.BigtableTableAdmin.DeleteTable]"],["DropRowRangeRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange][google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange]"],["EncryptionInfo","Encryption information for a given resource. If this resource is protected with customer managed encryption, the in-use Cloud Key Management Service (Cloud KMS) key version is specified along with its status."],["GcRule","Rule for determining which cells to delete during garbage collection."],["GenerateConsistencyTokenRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken][google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken]"],["GenerateConsistencyTokenResponse","Response message for [google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken][google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken]"],["GetAppProfileRequest","Request message for BigtableInstanceAdmin.GetAppProfile."],["GetBackupRequest","The request for [GetBackup][google.bigtable.admin.v2.BigtableTableAdmin.GetBackup]."],["GetClusterRequest","Request message for BigtableInstanceAdmin.GetCluster."],["GetInstanceRequest","Request message for BigtableInstanceAdmin.GetInstance."],["GetSnapshotRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.GetSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.GetSnapshot]"],["GetTableRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.GetTable][google.bigtable.admin.v2.BigtableTableAdmin.GetTable]"],["Instance","A collection of Bigtable [Tables][google.bigtable.admin.v2.Table] and the resources that serve them. All tables in an instance are served from all [Clusters][google.bigtable.admin.v2.Cluster] in the instance."],["ListAppProfilesRequest","Request message for BigtableInstanceAdmin.ListAppProfiles."],["ListAppProfilesResponse","Response message for BigtableInstanceAdmin.ListAppProfiles."],["ListBackupsRequest","The request for [ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups]."],["ListBackupsResponse","The response for [ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups]."],["ListClustersRequest","Request message for BigtableInstanceAdmin.ListClusters."],["ListClustersResponse","Response message for BigtableInstanceAdmin.ListClusters."],["ListInstancesRequest","Request message for BigtableInstanceAdmin.ListInstances."],["ListInstancesResponse","Response message for BigtableInstanceAdmin.ListInstances."],["ListSnapshotsRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots][google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots]"],["ListSnapshotsResponse","Response message for [google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots][google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots]"],["ListTablesRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.ListTables][google.bigtable.admin.v2.BigtableTableAdmin.ListTables]"],["ListTablesResponse","Response message for [google.bigtable.admin.v2.BigtableTableAdmin.ListTables][google.bigtable.admin.v2.BigtableTableAdmin.ListTables]"],["ModifyColumnFamiliesRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies][google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies]"],["OperationProgress","Encapsulates progress related information for a Cloud Bigtable long running operation."],["OptimizeRestoredTableMetadata","Metadata type for the long-running operation used to track the progress of optimizations performed on a newly restored table. This long-running operation is automatically created by the system after the successful completion of a table restore, and cannot be cancelled."],["PartialUpdateClusterMetadata","The metadata for the Operation returned by PartialUpdateCluster."],["PartialUpdateClusterRequest","Request message for BigtableInstanceAdmin.PartialUpdateCluster."],["PartialUpdateInstanceRequest","Request message for BigtableInstanceAdmin.PartialUpdateInstance."],["RestoreInfo","Information about a table restore."],["RestoreTableMetadata","Metadata type for the long-running operation returned by [RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable]."],["RestoreTableRequest","The request for [RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable]."],["Snapshot","A snapshot of a table at a particular time. A snapshot can be used as a checkpoint for data restoration or a data source for a new table."],["SnapshotTableMetadata","The metadata for the Operation returned by SnapshotTable."],["SnapshotTableRequest","Request message for [google.bigtable.admin.v2.BigtableTableAdmin.SnapshotTable][google.bigtable.admin.v2.BigtableTableAdmin.SnapshotTable]"],["Table","A collection of user data indexed by row, column, and timestamp. Each table is served using the resources of its parent cluster."],["UpdateAppProfileMetadata","The metadata for the Operation returned by UpdateAppProfile."],["UpdateAppProfileRequest","Request message for BigtableInstanceAdmin.UpdateAppProfile."],["UpdateBackupRequest","The request for [UpdateBackup][google.bigtable.admin.v2.BigtableTableAdmin.UpdateBackup]."],["UpdateClusterMetadata","The metadata for the Operation returned by UpdateCluster."],["UpdateInstanceMetadata","The metadata for the Operation returned by UpdateInstance."]]});