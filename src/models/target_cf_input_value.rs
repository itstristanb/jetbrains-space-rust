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
pub struct TargetCfInputValue {
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<Box<crate::models::TargetIdentifier>>,
}

impl TargetCfInputValue {
    pub fn new() -> TargetCfInputValue {
        TargetCfInputValue {
            target: None,
        }
    }
}


