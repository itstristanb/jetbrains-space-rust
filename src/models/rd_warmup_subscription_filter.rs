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
pub struct RdWarmupSubscriptionFilter {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<crate::models::PrProject>>,
    #[serde(rename = "repositoryName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<Option<String>>,
    #[serde(rename = "branchSpec", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub branch_spec: Option<Option<Vec<String>>>,
}

impl RdWarmupSubscriptionFilter {
    pub fn new() -> RdWarmupSubscriptionFilter {
        RdWarmupSubscriptionFilter {
            project: None,
            repository_name: None,
            branch_spec: None,
        }
    }
}


