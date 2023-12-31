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
pub struct IssueWebhookCustomFieldUpdate {
    #[serde(rename = "customField")]
    pub custom_field: Box<crate::models::CustomField>,
    #[serde(rename = "mod", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#mod: Option<Option<Box<crate::models::IssueWebhookCustomFieldUpdateMod>>>,
}

impl IssueWebhookCustomFieldUpdate {
    pub fn new(custom_field: crate::models::CustomField) -> IssueWebhookCustomFieldUpdate {
        IssueWebhookCustomFieldUpdate {
            custom_field: Box::new(custom_field),
            r#mod: None,
        }
    }
}


