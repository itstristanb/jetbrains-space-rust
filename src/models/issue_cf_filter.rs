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
pub struct IssueCfFilter {
    #[serde(rename = "values")]
    pub values: Vec<crate::models::IssueCfInputValue>,
}

impl IssueCfFilter {
    pub fn new(values: Vec<crate::models::IssueCfInputValue>) -> IssueCfFilter {
        IssueCfFilter {
            values,
        }
    }
}


