initSidebarItems({"mod":[["admission_rule","Nested message and enum types in `AdmissionRule`."],["attestor","Nested message and enum types in `Attestor`."],["attestor_public_key","Nested message and enum types in `AttestorPublicKey`."],["binauthz_management_service_v1_beta1_client","Generated client implementations."],["continuous_validation_event","Nested message and enum types in `ContinuousValidationEvent`."],["pkix_public_key","Nested message and enum types in `PkixPublicKey`."],["policy","Nested message and enum types in `Policy`."],["system_policy_v1_beta1_client","Generated client implementations."]],"struct":[["AdmissionRule","An [admission rule][google.cloud.binaryauthorization.v1beta1.AdmissionRule] specifies either that all container images used in a pod creation request must be attested to by one or more [attestors][google.cloud.binaryauthorization.v1beta1.Attestor], that all pod creations will be allowed, or that all pod creations will be denied."],["AdmissionWhitelistPattern","An [admission allowlist pattern][google.cloud.binaryauthorization.v1beta1.AdmissionWhitelistPattern] exempts images from checks by [admission rules][google.cloud.binaryauthorization.v1beta1.AdmissionRule]."],["Attestor","An [attestor][google.cloud.binaryauthorization.v1beta1.Attestor] that attests to container image artifacts. An existing attestor cannot be modified except where indicated."],["AttestorPublicKey","An [attestor public key][google.cloud.binaryauthorization.v1beta1.AttestorPublicKey] that will be used to verify attestations signed by this attestor."],["ContinuousValidationEvent","Represents an auditing event from Continuous Validation."],["CreateAttestorRequest","Request message for [BinauthzManagementService.CreateAttestor][]."],["DeleteAttestorRequest","Request message for [BinauthzManagementService.DeleteAttestor][]."],["GetAttestorRequest","Request message for [BinauthzManagementService.GetAttestor][]."],["GetPolicyRequest","Request message for [BinauthzManagementService.GetPolicy][]."],["GetSystemPolicyRequest","Request to read the current system policy."],["ListAttestorsRequest","Request message for [BinauthzManagementService.ListAttestors][]."],["ListAttestorsResponse","Response message for [BinauthzManagementService.ListAttestors][]."],["PkixPublicKey","A public key in the PkixPublicKey format (see https://tools.ietf.org/html/rfc5280#section-4.1.2.7 for details). Public keys of this type are typically textually encoded using the PEM format."],["Policy","A [policy][google.cloud.binaryauthorization.v1beta1.Policy] for Binary Authorization."],["UpdateAttestorRequest","Request message for [BinauthzManagementService.UpdateAttestor][]."],["UpdatePolicyRequest","Request message for [BinauthzManagementService.UpdatePolicy][]."],["UserOwnedDrydockNote","An [user owned drydock note][google.cloud.binaryauthorization.v1beta1.UserOwnedDrydockNote] references a Drydock ATTESTATION_AUTHORITY Note created by the user."]]});