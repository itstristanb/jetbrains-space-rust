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
pub struct RdWarmupSubscriptionFilterIn {
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "repositoryName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<Option<String>>,
    #[serde(rename = "branchSpec", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub branch_spec: Option<Option<Vec<String>>>,
}

impl RdWarmupSubscriptionFilterIn {
    pub fn new(project: String) -> RdWarmupSubscriptionFilterIn {
        RdWarmupSubscriptionFilterIn {
            project,
            repository_name: None,
            branch_spec: None,
        }
    }
}


