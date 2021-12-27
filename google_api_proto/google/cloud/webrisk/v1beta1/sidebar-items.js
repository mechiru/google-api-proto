initSidebarItems({"enum":[["CompressionType","The ways in which threat entry sets can be compressed."],["ThreatType","The type of threat. This maps dirrectly to the threat list a threat may belong to."]],"mod":[["compute_threat_list_diff_request","Nested message and enum types in `ComputeThreatListDiffRequest`."],["compute_threat_list_diff_response","Nested message and enum types in `ComputeThreatListDiffResponse`."],["search_hashes_response","Nested message and enum types in `SearchHashesResponse`."],["search_uris_response","Nested message and enum types in `SearchUrisResponse`."],["web_risk_service_v1_beta1_client","Generated client implementations."]],"struct":[["ComputeThreatListDiffRequest","Describes an API diff request."],["ComputeThreatListDiffResponse",""],["RawHashes","The uncompressed threat entries in hash format. Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4 bytes, but some hashes are lengthened if they collide with the hash of a popular URI."],["RawIndices","A set of raw indices to remove from a local list."],["RiceDeltaEncoding","The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or compressed removal indices."],["SearchHashesRequest","Request to return full hashes matched by the provided hash prefixes."],["SearchHashesResponse",""],["SearchUrisRequest","Request to check URI entries against threatLists."],["SearchUrisResponse",""],["ThreatEntryAdditions","Contains the set of entries to add to a local database. May contain a combination of compressed and raw data in a single response."],["ThreatEntryRemovals","Contains the set of entries to remove from a local database."]]});