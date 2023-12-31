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
pub struct CustomFieldInputValue {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "value")]
    pub value: Box<crate::models::CfInputValue>,
}

impl CustomFieldInputValue {
    pub fn new(field_id: String, value: crate::models::CfInputValue) -> CustomFieldInputValue {
        CustomFieldInputValue {
            field_id,
            value: Box::new(value),
        }
    }
}


