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
pub struct StringCfConstraint {
    #[serde(rename = "min", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min: Option<Option<i32>>,
    #[serde(rename = "max", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max: Option<Option<i32>>,
    #[serde(rename = "pattern", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Option<String>>,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
}

impl StringCfConstraint {
    pub fn new() -> StringCfConstraint {
        StringCfConstraint {
            min: None,
            max: None,
            pattern: None,
            message: None,
        }
    }
}


