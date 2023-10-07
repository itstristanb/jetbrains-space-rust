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
pub struct EnvVar {
    #[serde(rename = "type")]
    pub r#type: crate::models::EnvVarType,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl EnvVar {
    pub fn new(r#type: crate::models::EnvVarType, key: String, value: String) -> EnvVar {
        EnvVar {
            r#type,
            key,
            value,
        }
    }
}


