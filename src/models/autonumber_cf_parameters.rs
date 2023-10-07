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
pub struct AutonumberCfParameters {
    #[serde(rename = "prefix")]
    pub prefix: String,
    #[serde(rename = "suffix")]
    pub suffix: String,
}

impl AutonumberCfParameters {
    pub fn new(prefix: String, suffix: String) -> AutonumberCfParameters {
        AutonumberCfParameters {
            prefix,
            suffix,
        }
    }
}


