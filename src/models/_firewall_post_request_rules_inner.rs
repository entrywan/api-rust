/*
 * Entrywan API
 *
 * Manage Entrywan resources programmatically using the API.  All API requests are authenticated using [IAM tokens](https://entrywan.com/docs#iam).  Tokens can be generated and retrieved from the [portal](https://portal.entrywan.com).  The portal itself is an API client that uses an unrestricted token to access resources for an account.  This documentation is generated using an OpenAPI 3.1.0 [specification](https://spec.openapis.org/oas/latest.html).  More information about OpenAPI can be found on its [site](https://openapis.org).  The current version of [Entrywan's API spec](https://entrywan.com/openapi.yaml) is also available for inspection.  On the left of this page are links to the <i>Endpoints</i> grouped by tag and <i>Schemas</i> the API exposes.  <i>Endpoints</i> are URLs that can be accessed with any HTTP client or device. <i>Schemas</i> are machine-readable data models that represent resources.  To learn more, have a look at the [documentation](https://entrywan.com/docs).  If you have any questions, contact [support](mailto:support@entrywan.com) or your account representative.
 *
 * The version of the OpenAPI document: v1beta
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallPostRequestRulesInner {
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "src", skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
}

impl FirewallPostRequestRulesInner {
    pub fn new() -> FirewallPostRequestRulesInner {
        FirewallPostRequestRulesInner {
            protocol: None,
            port: None,
            src: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "ucp")]
    Ucp,
    #[serde(rename = "udplite")]
    Udplite,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "icmpv6")]
    Icmpv6,
    #[serde(rename = "esp")]
    Esp,
    #[serde(rename = "ah")]
    Ah,
    #[serde(rename = "sctp")]
    Sctp,
    #[serde(rename = "mh")]
    Mh,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::All
    }
}

