initSidebarItems({"enum":[["AnnotationType","When an [Annotation][google.genomics.v1.Annotation] or [AnnotationSet][google.genomics.v1.AnnotationSet] is created, if `type` is not specified it will be set to `GENERIC`."],["InfoMergeOperation","Operations to be performed during import on Variant info fields. These operations are set for each info field in the info_merge_config map of ImportVariantsRequest, which is plumbed down to the MergeVariantRequests generated by the import job."]],"mod":[["annotation","Nested message and enum types in `Annotation`."],["annotation_service_v1_client","Generated client implementations."],["batch_create_annotations_response","Nested message and enum types in `BatchCreateAnnotationsResponse`."],["cigar_unit","Nested message and enum types in `CigarUnit`."],["dataset_service_v1_client","Generated client implementations."],["export_variant_set_request","Nested message and enum types in `ExportVariantSetRequest`."],["import_read_group_sets_request","Nested message and enum types in `ImportReadGroupSetsRequest`."],["import_variants_request","Nested message and enum types in `ImportVariantsRequest`."],["read_group","Nested message and enum types in `ReadGroup`."],["read_service_v1_client","Generated client implementations."],["reference_service_v1_client","Generated client implementations."],["search_annotations_request","Nested message and enum types in `SearchAnnotationsRequest`."],["streaming_read_service_client","Generated client implementations."],["streaming_variant_service_client","Generated client implementations."],["transcript","Nested message and enum types in `Transcript`."],["variant_annotation","Nested message and enum types in `VariantAnnotation`."],["variant_service_v1_client","Generated client implementations."],["variant_set_metadata","Nested message and enum types in `VariantSetMetadata`."]],"struct":[["Annotation","An annotation describes a region of reference genome. The value of an annotation may be one of several canonical types, supplemented by arbitrary info tags. An annotation is not inherently associated with a specific sample or individual (though a client could choose to use annotations in this way). Example canonical annotation types are `GENE` and `VARIANT`."],["AnnotationSet","An annotation set is a logical grouping of annotations that share consistent type information and provenance. Examples of annotation sets include ‘all genes from refseq’, and ‘all variant annotations from ClinVar’."],["BatchCreateAnnotationsRequest",""],["BatchCreateAnnotationsResponse",""],["CallSet","A call set is a collection of variant calls, typically for one sample. It belongs to a variant set."],["CigarUnit","A single CIGAR operation."],["CoverageBucket","A bucket over which read coverage has been precomputed. A bucket corresponds to a specific range of the reference sequence."],["CreateAnnotationRequest",""],["CreateAnnotationSetRequest",""],["CreateCallSetRequest",""],["CreateDatasetRequest",""],["CreateVariantRequest",""],["CreateVariantSetRequest","The CreateVariantSet request"],["Dataset","A Dataset is a collection of genomic data."],["DeleteAnnotationRequest",""],["DeleteAnnotationSetRequest",""],["DeleteCallSetRequest",""],["DeleteDatasetRequest",""],["DeleteReadGroupSetRequest",""],["DeleteVariantRequest",""],["DeleteVariantSetRequest","The delete variant set request."],["ExportReadGroupSetRequest","The read group set export request."],["ExportVariantSetRequest","The variant data export request."],["ExternalId",""],["GetAnnotationRequest",""],["GetAnnotationSetRequest",""],["GetCallSetRequest",""],["GetDatasetRequest",""],["GetReadGroupSetRequest",""],["GetReferenceRequest",""],["GetReferenceSetRequest",""],["GetVariantRequest",""],["GetVariantSetRequest","The variant set request."],["ImportReadGroupSetsRequest","The read group set import request."],["ImportReadGroupSetsResponse","The read group set import response."],["ImportVariantsRequest","The variant data import request."],["ImportVariantsResponse","The variant data import response."],["LinearAlignment","A linear alignment can be represented by one CIGAR string. Describes the mapped position and local alignment of the read to the reference."],["ListBasesRequest",""],["ListBasesResponse",""],["ListCoverageBucketsRequest",""],["ListCoverageBucketsResponse",""],["ListDatasetsRequest","The dataset list request."],["ListDatasetsResponse","The dataset list response."],["MergeVariantsRequest",""],["OperationEvent","An event that occurred during an [Operation][google.longrunning.Operation]."],["OperationMetadata","Metadata describing an [Operation][google.longrunning.Operation]."],["Position","An abstraction for referring to a genomic position, in relation to some already known reference. For now, represents a genomic position as a reference name, a base number on that reference (0-based), and a determination of forward or reverse strand."],["Range","A 0-based half-open genomic coordinate range for search requests."],["Read","A read alignment describes a linear alignment of a string of DNA to a [reference sequence][google.genomics.v1.Reference], in addition to metadata about the fragment (the molecule of DNA sequenced) and the read (the bases which were read by the sequencer). A read is equivalent to a line in a SAM file. A read belongs to exactly one read group and exactly one [read group set][google.genomics.v1.ReadGroupSet]."],["ReadGroup","A read group is all the data that’s processed the same way by the sequencer."],["ReadGroupSet","A read group set is a logical collection of read groups, which are collections of reads produced by a sequencer. A read group set typically models reads corresponding to one sample, sequenced one way, and aligned one way."],["Reference","A reference is a canonical assembled DNA sequence, intended to act as a reference coordinate space for other genomic annotations. A single reference might represent the human chromosome 1 or mitochandrial DNA, for instance. A reference belongs to one or more reference sets."],["ReferenceBound","ReferenceBound records an upper bound for the starting coordinate of variants in a particular reference."],["ReferenceSet","A reference set is a set of references which typically comprise a reference assembly for a species, such as `GRCh38` which is representative of the human genome. A reference set defines a common coordinate space for comparing reference-aligned experimental data. A reference set contains 1 or more references."],["SearchAnnotationSetsRequest",""],["SearchAnnotationSetsResponse",""],["SearchAnnotationsRequest",""],["SearchAnnotationsResponse",""],["SearchCallSetsRequest","The call set search request."],["SearchCallSetsResponse","The call set search response."],["SearchReadGroupSetsRequest","The read group set search request."],["SearchReadGroupSetsResponse","The read group set search response."],["SearchReadsRequest","The read search request."],["SearchReadsResponse","The read search response."],["SearchReferenceSetsRequest",""],["SearchReferenceSetsResponse",""],["SearchReferencesRequest",""],["SearchReferencesResponse",""],["SearchVariantSetsRequest","The search variant sets request."],["SearchVariantSetsResponse","The search variant sets response."],["SearchVariantsRequest","The variant search request."],["SearchVariantsResponse","The variant search response."],["StreamReadsRequest","The stream reads request."],["StreamReadsResponse",""],["StreamVariantsRequest","The stream variants request."],["StreamVariantsResponse",""],["Transcript","A transcript represents the assertion that a particular region of the reference genome may be transcribed as RNA."],["UndeleteDatasetRequest",""],["UpdateAnnotationRequest",""],["UpdateAnnotationSetRequest",""],["UpdateCallSetRequest",""],["UpdateDatasetRequest",""],["UpdateReadGroupSetRequest",""],["UpdateVariantRequest",""],["UpdateVariantSetRequest",""],["Variant","A variant represents a change in DNA sequence relative to a reference sequence. For example, a variant could represent a SNP or an insertion. Variants belong to a variant set."],["VariantAnnotation",""],["VariantCall","A call represents the determination of genotype with respect to a particular variant. It may include associated information such as quality and phasing. For example, a call might assign a probability of 0.32 to the occurrence of a SNP named rs1234 in a call set with the name NA12345."],["VariantSet","A variant set is a collection of call sets and variants. It contains summary statistics of those contents. A variant set belongs to a dataset."],["VariantSetMetadata","Metadata describes a single piece of variant call metadata. These data include a top level key and either a single value string (value) or a list of key-value pairs (info.) Value and info are mutually exclusive."]]});