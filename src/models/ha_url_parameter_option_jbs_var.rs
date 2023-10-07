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
pub struct HaUrlParameterOptionJbsVar {
    #[serde(rename = "parameter")]
    pub parameter: Box<crate::models::HaField>,
    #[serde(rename = "parameters")]
    pub parameters: Vec<crate::models::HaField>,
    #[serde(rename = "prefixRequired")]
    pub prefix_required: bool,
    #[serde(rename = "optionName")]
    pub option_name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<crate::models::HaDescription>>,
    #[serde(rename = "deprecation", skip_serializing_if = "Option::is_none")]
    pub deprecation: Option<Box<crate::models::HaDeprecation>>,
    #[serde(rename = "experimental", skip_serializing_if = "Option::is_none")]
    pub experimental: Option<Box<crate::models::HaExperimental>>,
    #[serde(rename = "featureFlag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<Option<String>>,
}

impl HaUrlParameterOptionJbsVar {
    pub fn new(parameter: crate::models::HaField, parameters: Vec<crate::models::HaField>, prefix_required: bool, option_name: String) -> HaUrlParameterOptionJbsVar {
        HaUrlParameterOptionJbsVar {
            parameter: Box::new(parameter),
            parameters,
            prefix_required,
            option_name,
            description: None,
            deprecation: None,
            experimental: None,
            feature_flag: None,
        }
    }
}

