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
pub struct IssueWebhookCustomFieldUpdateMod {
    #[serde(rename = "old")]
    pub old: Box<crate::models::CfValue>,
    #[serde(rename = "new")]
    pub new: Box<crate::models::CfValue>,
}

impl IssueWebhookCustomFieldUpdateMod {
    pub fn new(old: crate::models::CfValue, new: crate::models::CfValue) -> IssueWebhookCustomFieldUpdateMod {
        IssueWebhookCustomFieldUpdateMod {
            old: Box::new(old),
            new: Box::new(new),
        }
    }
}


