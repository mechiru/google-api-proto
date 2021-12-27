initSidebarItems({"enum":[["IpProtocolEnum","The IP protocol to which this rule applies. For protocol forwarding, valid options are TCP, UDP, ESP, AH, SCTP, ICMP and L3_DEFAULT. The valid IP protocols are different for different load balancing products as described in Load balancing features."],["IpVersion","The IP Version that will be used by this forwarding rule. Valid options are IPV4 or IPV6. This can only be specified for an external global forwarding rule."],["LoadBalancingScheme","Specifies the forwarding rule type. For more information about forwarding rules, refer to Forwarding rule concepts."],["NetworkTier","This signifies the networking tier used for configuring this load balancer and can only take the following values: PREMIUM, STANDARD. For regional ForwardingRule, the valid values are PREMIUM and STANDARD. For GlobalForwardingRule, the valid value is PREMIUM. If this field is not specified, it is assumed to be PREMIUM. If IPAddress is specified, this value must be equal to the networkTier of the Address."],["PscConnectionStatus",""]]});