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
pub struct PrRepositoryInfo {
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "latestActivity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latest_activity: Option<Option<String>>,
    #[serde(rename = "proxyPushNotification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub proxy_push_notification: Option<Option<String>>,
    #[serde(rename = "proxyPushNotificationBody", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub proxy_push_notification_body: Option<Option<String>>,
    #[serde(rename = "state")]
    pub state: crate::models::RepositoryState,
    #[serde(rename = "initProgress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub init_progress: Option<Option<String>>,
    #[serde(rename = "readmeName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub readme_name: Option<Option<String>>,
    #[serde(rename = "monthlyActivity", skip_serializing_if = "Option::is_none")]
    pub monthly_activity: Option<Box<crate::models::RepositoryActivity>>,
    #[serde(rename = "defaultBranch", skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<Box<crate::models::BranchInfo>>,
}

impl PrRepositoryInfo {
    pub fn new(name: String, description: String, state: crate::models::RepositoryState) -> PrRepositoryInfo {
        PrRepositoryInfo {
            id: None,
            name,
            description,
            latest_activity: None,
            proxy_push_notification: None,
            proxy_push_notification_body: None,
            state,
            init_progress: None,
            readme_name: None,
            monthly_activity: None,
            default_branch: None,
        }
    }
}


