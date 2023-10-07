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
pub struct CustomField {
    #[serde(rename = "extendedType")]
    pub extended_type: Box<crate::models::ExtendedType>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub key: Option<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::CfType>,
    #[serde(rename = "constraint", skip_serializing_if = "Option::is_none")]
    pub constraint: Option<Box<crate::models::CfConstraint>>,
    #[serde(rename = "required")]
    pub required: bool,
    #[serde(rename = "private")]
    pub private: bool,
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<crate::models::AccessType>,
    #[serde(rename = "defaultValue")]
    pub default_value: Box<crate::models::CfValue>,
    #[serde(rename = "order")]
    pub order: i32,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<crate::models::ExtendedTypeScope>>,
    #[serde(rename = "deleted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Option<bool>>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<crate::models::CfParameters>>,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl CustomField {
    pub fn new(extended_type: crate::models::ExtendedType, id: String, name: String, r#type: crate::models::CfType, required: bool, private: bool, default_value: crate::models::CfValue, order: i32, archived: bool) -> CustomField {
        CustomField {
            extended_type: Box::new(extended_type),
            id,
            name,
            description: None,
            key: None,
            r#type: Box::new(r#type),
            constraint: None,
            required,
            private,
            access: None,
            default_value: Box::new(default_value),
            order,
            scope: None,
            deleted: None,
            parameters: None,
            archived,
        }
    }
}


