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
pub struct CustomFieldsTypeKeyFieldsReorderPostRequest {
    #[serde(rename = "customFieldOrder")]
    pub custom_field_order: Vec<String>,
    #[serde(rename = "scope")]
    pub scope: Box<crate::models::ExtendedTypeScope>,
}

impl CustomFieldsTypeKeyFieldsReorderPostRequest {
    pub fn new(custom_field_order: Vec<String>, scope: crate::models::ExtendedTypeScope) -> CustomFieldsTypeKeyFieldsReorderPostRequest {
        CustomFieldsTypeKeyFieldsReorderPostRequest {
            custom_field_order,
            scope: Box::new(scope),
        }
    }
}

