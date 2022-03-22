initSidebarItems({"enum":[["VersionView","The view, which determines what version information is returned in a response."]],"mod":[["apt_artifact","Nested message and enum types in `AptArtifact`."],["artifact_registry_client","Generated client implementations."],["hash","Nested message and enum types in `Hash`."],["import_apt_artifacts_error_info","Nested message and enum types in `ImportAptArtifactsErrorInfo`."],["import_apt_artifacts_request","Nested message and enum types in `ImportAptArtifactsRequest`."],["import_yum_artifacts_error_info","Nested message and enum types in `ImportYumArtifactsErrorInfo`."],["import_yum_artifacts_request","Nested message and enum types in `ImportYumArtifactsRequest`."],["project_settings","Nested message and enum types in `ProjectSettings`."],["repository","Nested message and enum types in `Repository`."],["yum_artifact","Nested message and enum types in `YumArtifact`."]],"struct":[["AptArtifact","A detailed representation of an Apt artifact. Information in the record is derived from the archive’s control file. See https://www.debian.org/doc/debian-policy/ch-controlfields.html"],["CreateRepositoryRequest","The request to create a new repository."],["CreateTagRequest","The request to create a new tag."],["DeletePackageRequest","The request to delete a package."],["DeleteRepositoryRequest","The request to delete a repository."],["DeleteTagRequest","The request to delete a tag."],["DeleteVersionRequest","The request to delete a version."],["DockerImage","DockerImage represents a docker artifact. The following fields are returned as untyped metadata in the Version resource, using camelcase keys (i.e. metadata.imageSizeBytes):"],["File","Files store content that is potentially associated with Packages or Versions."],["GetDockerImageRequest","The request to get docker images."],["GetFileRequest","The request to retrieve a file."],["GetPackageRequest","The request to retrieve a package."],["GetProjectSettingsRequest","Gets the redirection status for a project."],["GetRepositoryRequest","The request to retrieve a repository."],["GetTagRequest","The request to retrieve a tag."],["GetVersionRequest","The request to retrieve a version."],["Hash","A hash of file content."],["ImportAptArtifactsErrorInfo","Error information explaining why a package was not imported."],["ImportAptArtifactsGcsSource","Google Cloud Storage location where the artifacts currently reside."],["ImportAptArtifactsMetadata","The operation metadata for importing artifacts."],["ImportAptArtifactsRequest","The request to import new apt artifacts."],["ImportAptArtifactsResponse","The response message from importing APT artifacts."],["ImportYumArtifactsErrorInfo","Error information explaining why a package was not imported."],["ImportYumArtifactsGcsSource","Google Cloud Storage location where the artifacts currently reside."],["ImportYumArtifactsMetadata","The operation metadata for importing artifacts."],["ImportYumArtifactsRequest","The request to import new yum artifacts."],["ImportYumArtifactsResponse","The response message from importing YUM artifacts."],["ListDockerImagesRequest","The request to list docker images."],["ListDockerImagesResponse","The response from listing docker images."],["ListFilesRequest","The request to list files."],["ListFilesResponse","The response from listing files."],["ListPackagesRequest","The request to list packages."],["ListPackagesResponse","The response from listing packages."],["ListRepositoriesRequest","The request to list repositories."],["ListRepositoriesResponse","The response from listing repositories."],["ListTagsRequest","The request to list tags."],["ListTagsResponse","The response from listing tags."],["ListVersionsRequest","The request to list versions."],["ListVersionsResponse","The response from listing versions."],["OperationMetadata","Metadata type for longrunning-operations, currently empty."],["Package","Packages are named collections of versions."],["ProjectSettings","The Artifact Registry settings that apply to a Project."],["Repository","A Repository for storing artifacts with a specific format."],["Tag","Tags point to a version and represent an alternative name that can be used to access the version."],["UpdateProjectSettingsRequest","Sets the settings of the project."],["UpdateRepositoryRequest","The request to update a repository."],["UpdateTagRequest","The request to create or update a tag."],["Version","The body of a version resource. A version resource represents a collection of components, such as files and other data. This may correspond to a version in many package management schemes."],["YumArtifact","A detailed representation of a Yum artifact."]]});