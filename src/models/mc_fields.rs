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
pub struct McFields {
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::McFieldsFieldsInner>,
}

impl McFields {
    pub fn new(fields: Vec<crate::models::McFieldsFieldsInner>) -> McFields {
        McFields {
            fields,
        }
    }
}


