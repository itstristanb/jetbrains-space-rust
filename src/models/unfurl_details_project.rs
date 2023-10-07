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
pub struct UnfurlDetailsProject {
    #[serde(rename = "project")]
    pub project: Box<crate::models::PrProject>,
    #[serde(rename = "strikeThrough", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub strike_through: Option<Option<bool>>,
}

impl UnfurlDetailsProject {
    pub fn new(project: crate::models::PrProject) -> UnfurlDetailsProject {
        UnfurlDetailsProject {
            project: Box::new(project),
            strike_through: None,
        }
    }
}


