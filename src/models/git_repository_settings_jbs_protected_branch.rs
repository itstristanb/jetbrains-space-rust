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
pub struct GitRepositorySettingsJbsProtectedBranch {
    #[serde(rename = "pattern")]
    pub pattern: Vec<String>,
    #[serde(rename = "regex", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub regex: Option<Option<bool>>,
    #[serde(rename = "allowCreate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allow_create: Option<Option<Vec<String>>>,
    #[serde(rename = "allowPush", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allow_push: Option<Option<Vec<String>>>,
    #[serde(rename = "allowDelete", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allow_delete: Option<Option<Vec<String>>>,
    #[serde(rename = "allowForcePush", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allow_force_push: Option<Option<Vec<String>>>,
    #[serde(rename = "qualityGate", skip_serializing_if = "Option::is_none")]
    pub quality_gate: Option<Box<crate::models::GitRepositorySettingsJbsQualityGate>>,
    #[serde(rename = "safeMerge", skip_serializing_if = "Option::is_none")]
    pub safe_merge: Option<Box<crate::models::GitRepositorySettingsJbsSafeMerge>>,
    #[serde(rename = "linearHistory", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub linear_history: Option<Option<bool>>,
}

impl GitRepositorySettingsJbsProtectedBranch {
    pub fn new(pattern: Vec<String>) -> GitRepositorySettingsJbsProtectedBranch {
        GitRepositorySettingsJbsProtectedBranch {
            pattern,
            regex: None,
            allow_create: None,
            allow_push: None,
            allow_delete: None,
            allow_force_push: None,
            quality_gate: None,
            safe_merge: None,
            linear_history: None,
        }
    }
}

