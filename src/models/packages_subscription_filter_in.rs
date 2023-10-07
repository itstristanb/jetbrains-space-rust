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
pub struct PackagesSubscriptionFilterIn {
    #[serde(rename = "project", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project: Option<Option<String>>,
    #[serde(rename = "repository", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub repository: Option<Option<String>>,
    #[serde(rename = "namePattern")]
    pub name_pattern: Vec<String>,
    #[serde(rename = "versionPattern", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub version_pattern: Option<Option<String>>,
}

impl PackagesSubscriptionFilterIn {
    pub fn new(name_pattern: Vec<String>) -> PackagesSubscriptionFilterIn {
        PackagesSubscriptionFilterIn {
            project: None,
            repository: None,
            name_pattern,
            version_pattern: None,
        }
    }
}

