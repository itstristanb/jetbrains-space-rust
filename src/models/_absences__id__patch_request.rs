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
pub struct AbsencesIdPatchRequest {
    #[serde(rename = "member", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub member: Option<Option<String>>,
    #[serde(rename = "reason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<String>>,
    #[serde(rename = "since", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub since: Option<Option<String>>,
    #[serde(rename = "till", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub till: Option<Option<String>>,
    #[serde(rename = "available")]
    pub available: bool,
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    #[serde(rename = "customFieldValues", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_field_values: Option<Option<Vec<crate::models::CustomFieldInputValue>>>,
}

impl AbsencesIdPatchRequest {
    pub fn new(available: bool) -> AbsencesIdPatchRequest {
        AbsencesIdPatchRequest {
            member: None,
            reason: None,
            description: None,
            location: None,
            since: None,
            till: None,
            available,
            icon: None,
            custom_field_values: None,
        }
    }
}


