initSidebarItems({"enum":[["EnforceOnKey","Determines the key to enforce the rate_limit_threshold on. Possible values are: - ALL: A single rate limit threshold is applied to all the requests matching this rule. This is the default value if this field ‘enforce_on_key’ is not configured. - IP: The source IP address of the request is the key. Each IP has this limit enforced separately. - HTTP_HEADER: The value of the HTTP header whose name is configured under “enforce_on_key_name”. The key value is truncated to the first 128 bytes of the header value. If no such header is present in the request, the key type defaults to ALL. - XFF_IP: The first IP address (i.e. the originating client IP address) specified in the list of IPs under X-Forwarded-For HTTP header. If no such header is present or the value is not a valid IP, the key type defaults to ALL. - HTTP_COOKIE: The value of the HTTP cookie whose name is configured under “enforce_on_key_name”. The key value is truncated to the first 128 bytes of the cookie value. If no such cookie is present in the request, the key type defaults to ALL."]]});