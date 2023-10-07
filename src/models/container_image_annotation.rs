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
pub struct ContainerImageAnnotation {
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<i64>>,
    #[serde(rename = "buildName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub build_name: Option<Option<String>>,
    #[serde(rename = "buildUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub build_url: Option<Option<String>>,
    #[serde(rename = "revision", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Option<String>>,
    #[serde(rename = "vendor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<Option<String>>,
    #[serde(rename = "documentationUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<Option<String>>,
    #[serde(rename = "licenses", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Option<String>>,
}

impl ContainerImageAnnotation {
    pub fn new() -> ContainerImageAnnotation {
        ContainerImageAnnotation {
            created: None,
            build_name: None,
            build_url: None,
            revision: None,
            vendor: None,
            documentation_url: None,
            licenses: None,
        }
    }
}


