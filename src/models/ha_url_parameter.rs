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
pub struct HaUrlParameter {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "options")]
    pub options: Vec<crate::models::HaUrlParameterOption>,
    #[serde(rename = "deprecation", skip_serializing_if = "Option::is_none")]
    pub deprecation: Option<Box<crate::models::HaDeprecation>>,
    #[serde(rename = "experimental", skip_serializing_if = "Option::is_none")]
    pub experimental: Option<Box<crate::models::HaExperimental>>,
}

impl HaUrlParameter {
    pub fn new(id: String, name: String, options: Vec<crate::models::HaUrlParameterOption>) -> HaUrlParameter {
        HaUrlParameter {
            id,
            name,
            options,
            deprecation: None,
            experimental: None,
        }
    }
}


