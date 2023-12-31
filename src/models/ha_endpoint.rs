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
pub struct HaEndpoint {
    #[serde(rename = "resource")]
    pub resource: Box<crate::models::HaResource>,
    #[serde(rename = "method")]
    pub method: crate::models::HaMethod,
    #[serde(rename = "parameters")]
    pub parameters: Vec<crate::models::HaParameter>,
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request_body: Option<Box<crate::models::HaTypeJbsObject>>,
    #[serde(rename = "responseBody", skip_serializing_if = "Option::is_none")]
    pub response_body: Option<Box<crate::models::HaType>>,
    #[serde(rename = "path")]
    pub path: Box<crate::models::HaPath>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "doc", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub doc: Option<Option<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<crate::models::HaDescription>>,
    #[serde(rename = "deprecation", skip_serializing_if = "Option::is_none")]
    pub deprecation: Option<Box<crate::models::HaDeprecation>>,
    #[serde(rename = "experimental", skip_serializing_if = "Option::is_none")]
    pub experimental: Option<Box<crate::models::HaExperimental>>,
    #[serde(rename = "rights", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rights: Option<Option<Vec<crate::models::HaRight>>>,
    #[serde(rename = "featureFlag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<Option<String>>,
}

impl HaEndpoint {
    pub fn new(resource: crate::models::HaResource, method: crate::models::HaMethod, parameters: Vec<crate::models::HaParameter>, path: crate::models::HaPath, display_name: String, function_name: String) -> HaEndpoint {
        HaEndpoint {
            resource: Box::new(resource),
            method,
            parameters,
            request_body: None,
            response_body: None,
            path: Box::new(path),
            display_name,
            function_name,
            doc: None,
            description: None,
            deprecation: None,
            experimental: None,
            rights: None,
            feature_flag: None,
        }
    }
}


