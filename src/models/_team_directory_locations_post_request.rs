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
pub struct TeamDirectoryLocationsPostRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "timezone", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Option<String>>,
    #[serde(rename = "workdays", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub workdays: Option<Option<Vec<i32>>>,
    #[serde(rename = "phones", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phones: Option<Option<Vec<String>>>,
    #[serde(rename = "emails", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Option<Vec<String>>>,
    #[serde(rename = "equipment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub equipment: Option<Option<Vec<String>>>,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    #[serde(rename = "parentId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<String>>,
    #[serde(rename = "capacity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<Option<i32>>,
}

impl TeamDirectoryLocationsPostRequest {
    pub fn new(name: String) -> TeamDirectoryLocationsPostRequest {
        TeamDirectoryLocationsPostRequest {
            name,
            timezone: None,
            workdays: None,
            phones: None,
            emails: None,
            equipment: None,
            description: None,
            address: None,
            r#type: None,
            parent_id: None,
            capacity: None,
        }
    }
}

