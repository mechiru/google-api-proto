initSidebarItems({"enum":[["ChannelPartnerLinkState","ChannelPartnerLinkState represents state of a channel partner link."],["ChannelPartnerLinkView","The level of granularity the [ChannelPartnerLink][google.cloud.channel.v1.ChannelPartnerLink] will display."],["MediaType","Type of media used."],["PaymentPlan","Describes how the reseller will be billed."],["PaymentType","Specifies when the payment needs to happen."],["PeriodType","Period Type."],["PromotionalOrderType","Constraints type for Promotional offers."],["ResourceType","Represents the type for a monetizable resource(any entity on which billing happens). For example, this could be MINUTES for Google Voice and GB for Google Drive. One SKU can map to multiple monetizable resources."]],"mod":[["cloud_channel_service_client","Generated client implementations."],["cloud_identity_info","Nested message and enum types in `CloudIdentityInfo`."],["customer_event","Nested message and enum types in `CustomerEvent`."],["edu_data","Nested message and enum types in `EduData`."],["entitlement","Nested message and enum types in `Entitlement`."],["entitlement_event","Nested message and enum types in `EntitlementEvent`."],["import_customer_request","Nested message and enum types in `ImportCustomerRequest`."],["list_purchasable_offers_request","Nested message and enum types in `ListPurchasableOffersRequest`."],["list_purchasable_skus_request","Nested message and enum types in `ListPurchasableSkusRequest`."],["list_transferable_offers_request","Nested message and enum types in `ListTransferableOffersRequest`."],["list_transferable_skus_request","Nested message and enum types in `ListTransferableSkusRequest`."],["operation_metadata","Nested message and enum types in `OperationMetadata`."],["parameter_definition","Nested message and enum types in `ParameterDefinition`."],["subscriber_event","Nested message and enum types in `SubscriberEvent`."],["transfer_eligibility","Nested message and enum types in `TransferEligibility`."],["value","Nested message and enum types in `Value`."]],"struct":[["ActivateEntitlementRequest","Request message for [CloudChannelService.ActivateEntitlement][google.cloud.channel.v1.CloudChannelService.ActivateEntitlement]."],["AdminUser","Information needed to create an Admin User for Google Workspace."],["AssociationInfo","Association links that an entitlement has to other entitlements."],["CancelEntitlementRequest","Request message for [CloudChannelService.CancelEntitlement][google.cloud.channel.v1.CloudChannelService.CancelEntitlement]."],["ChangeOfferRequest","Request message for [CloudChannelService.ChangeOffer][google.cloud.channel.v1.CloudChannelService.ChangeOffer]."],["ChangeParametersRequest","Request message for [CloudChannelService.ChangeParametersRequest][]."],["ChangeRenewalSettingsRequest","Request message for [CloudChannelService.ChangeRenewalSettings][google.cloud.channel.v1.CloudChannelService.ChangeRenewalSettings]."],["ChannelPartnerLink","Entity representing a link between distributors and their indirect resellers in an n-tier resale channel."],["CheckCloudIdentityAccountsExistRequest","Request message for [CloudChannelService.CheckCloudIdentityAccountsExist][google.cloud.channel.v1.CloudChannelService.CheckCloudIdentityAccountsExist]."],["CheckCloudIdentityAccountsExistResponse","Response message for [CloudChannelService.CheckCloudIdentityAccountsExist][google.cloud.channel.v1.CloudChannelService.CheckCloudIdentityAccountsExist]."],["CloudIdentityCustomerAccount","Entity representing a Cloud Identity account that may be associated with a Channel Services API partner."],["CloudIdentityInfo","Cloud Identity information for the Cloud Channel Customer."],["CommitmentSettings","Commitment settings for commitment-based offers."],["Constraints","Represents the constraints for buying the Offer."],["ContactInfo","Contact information for a customer account."],["CreateChannelPartnerLinkRequest","Request message for [CloudChannelService.CreateChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.CreateChannelPartnerLink]"],["CreateCustomerRequest","Request message for [CloudChannelService.CreateCustomer][google.cloud.channel.v1.CloudChannelService.CreateCustomer]"],["CreateEntitlementRequest","Request message for [CloudChannelService.CreateEntitlement][google.cloud.channel.v1.CloudChannelService.CreateEntitlement]"],["Customer","Entity representing a customer of a reseller or distributor."],["CustomerConstraints","Represents constraints required to purchase the Offer for a customer."],["CustomerEvent","Represents Pub/Sub message content describing customer update."],["DeleteCustomerRequest","Request message for [CloudChannelService.DeleteCustomer][google.cloud.channel.v1.CloudChannelService.DeleteCustomer]."],["EduData","Required Edu Attributes"],["Entitlement","An entitlement is a representation of a customer’s ability to use a service."],["EntitlementEvent","Represents Pub/Sub message content describing entitlement update."],["GetChannelPartnerLinkRequest","Request message for [CloudChannelService.GetChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.GetChannelPartnerLink]."],["GetCustomerRequest","Request message for [CloudChannelService.GetCustomer][google.cloud.channel.v1.CloudChannelService.GetCustomer]."],["GetEntitlementRequest","Request message for [CloudChannelService.GetEntitlement][google.cloud.channel.v1.CloudChannelService.GetEntitlement]."],["ImportCustomerRequest","Request message for [CloudChannelService.ImportCustomer][google.cloud.channel.v1.CloudChannelService.ImportCustomer]"],["ListChannelPartnerLinksRequest","Request message for [CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks]"],["ListChannelPartnerLinksResponse","Response message for [CloudChannelService.ListChannelPartnerLinks][google.cloud.channel.v1.CloudChannelService.ListChannelPartnerLinks]."],["ListCustomersRequest","Request message for [CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers]"],["ListCustomersResponse","Response message for [CloudChannelService.ListCustomers][google.cloud.channel.v1.CloudChannelService.ListCustomers]."],["ListEntitlementsRequest","Request message for [CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements]"],["ListEntitlementsResponse","Response message for [CloudChannelService.ListEntitlements][google.cloud.channel.v1.CloudChannelService.ListEntitlements]."],["ListOffersRequest","Request message for ListOffers."],["ListOffersResponse","Response message for ListOffers."],["ListProductsRequest","Request message for ListProducts."],["ListProductsResponse","Response message for ListProducts."],["ListPurchasableOffersRequest","Request message for ListPurchasableOffers."],["ListPurchasableOffersResponse","Response message for ListPurchasableOffers."],["ListPurchasableSkusRequest","Request message for ListPurchasableSkus."],["ListPurchasableSkusResponse","Response message for ListPurchasableSkus."],["ListSkusRequest","Request message for ListSkus."],["ListSkusResponse","Response message for ListSkus."],["ListSubscribersRequest","Request Message for ListSubscribers."],["ListSubscribersResponse","Response Message for ListSubscribers."],["ListTransferableOffersRequest","Request message for [CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers]"],["ListTransferableOffersResponse","Response message for [CloudChannelService.ListTransferableOffers][google.cloud.channel.v1.CloudChannelService.ListTransferableOffers]."],["ListTransferableSkusRequest","Request message for [CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus]"],["ListTransferableSkusResponse","Response message for [CloudChannelService.ListTransferableSkus][google.cloud.channel.v1.CloudChannelService.ListTransferableSkus]."],["LookupOfferRequest","Request message for LookupOffer."],["MarketingInfo","Represents the marketing information for a Product, SKU or Offer."],["Media","Represents media information."],["Offer","Represents an offer made to resellers for purchase. An offer is associated with a [Sku][google.cloud.channel.v1.Sku], has a plan for payment, a price, and defines the constraints for buying."],["OperationMetadata","Provides contextual information about a [google.longrunning.Operation][google.longrunning.Operation]."],["Parameter","Definition for extended entitlement parameters."],["ParameterDefinition","Parameter’s definition. Specifies what parameter is required to use the current Offer to purchase."],["Period","Represents period in days/months/years."],["Plan","The payment plan for the Offer. Describes how to make a payment."],["Price","Represents the price of the Offer."],["PriceByResource","Represents price by resource type."],["PricePhase","Specifies the price by the duration of months. For example, a 20% discount for the first six months, then a 10% discount starting on the seventh month."],["PriceTier","Defines price at resource tier level. For example, an offer with following definition :"],["Product","A Product is the entity a customer uses when placing an order. For example, Google Workspace, Google Voice, etc."],["ProvisionCloudIdentityRequest","Request message for [CloudChannelService.ProvisionCloudIdentity][google.cloud.channel.v1.CloudChannelService.ProvisionCloudIdentity]"],["ProvisionedService","Service provisioned for an entitlement."],["PurchasableOffer","Offer that you can purchase for a customer. This is used in the ListPurchasableOffer API response."],["PurchasableSku","SKU that you can purchase. This is used in ListPurchasableSku API response."],["RegisterSubscriberRequest","Request Message for RegisterSubscriber."],["RegisterSubscriberResponse","Response Message for RegisterSubscriber."],["RenewalSettings","Renewal settings for renewable Offers."],["Sku","Represents a product’s purchasable Stock Keeping Unit (SKU). SKUs represent the different variations of the product. For example, Google Workspace Business Standard and Google Workspace Business Plus are Google Workspace product SKUs."],["StartPaidServiceRequest","Request message for [CloudChannelService.StartPaidService][google.cloud.channel.v1.CloudChannelService.StartPaidService]."],["SubscriberEvent","Represents information which resellers will get as part of notification from Pub/Sub."],["SuspendEntitlementRequest","Request message for [CloudChannelService.SuspendEntitlement][google.cloud.channel.v1.CloudChannelService.SuspendEntitlement]."],["TransferEligibility","Specifies transfer eligibility of a SKU."],["TransferEntitlementsRequest","Request message for [CloudChannelService.TransferEntitlements][google.cloud.channel.v1.CloudChannelService.TransferEntitlements]."],["TransferEntitlementsResponse","Response message for [CloudChannelService.TransferEntitlements][google.cloud.channel.v1.CloudChannelService.TransferEntitlements]. This is put in the response field of google.longrunning.Operation."],["TransferEntitlementsToGoogleRequest","Request message for [CloudChannelService.TransferEntitlementsToGoogle][google.cloud.channel.v1.CloudChannelService.TransferEntitlementsToGoogle]."],["TransferableOffer","TransferableOffer represents an Offer that can be used in Transfer. Read-only."],["TransferableSku","TransferableSku represents information a reseller needs to view existing provisioned services for a customer that they do not own. Read-only."],["TrialSettings","Settings for trial offers."],["UnregisterSubscriberRequest","Request Message for UnregisterSubscriber."],["UnregisterSubscriberResponse","Response Message for UnregisterSubscriber."],["UpdateChannelPartnerLinkRequest","Request message for [CloudChannelService.UpdateChannelPartnerLink][google.cloud.channel.v1.CloudChannelService.UpdateChannelPartnerLink]"],["UpdateCustomerRequest","Request message for [CloudChannelService.UpdateCustomer][google.cloud.channel.v1.CloudChannelService.UpdateCustomer]."],["Value","Data type and value of a parameter."]]});