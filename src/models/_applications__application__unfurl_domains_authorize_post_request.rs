/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationsApplicationUnfurlDomainsAuthorizePostRequest {
    #[serde(rename = "domains")]
    pub domains: Vec<String>,
    #[serde(rename = "approve")]
    pub approve: bool,
}

impl ApplicationsApplicationUnfurlDomainsAuthorizePostRequest {
    pub fn new(domains: Vec<String>, approve: bool) -> ApplicationsApplicationUnfurlDomainsAuthorizePostRequest {
        ApplicationsApplicationUnfurlDomainsAuthorizePostRequest {
            domains,
            approve,
        }
    }
}


