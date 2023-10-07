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
pub struct ApplicationUnfurlContentJbsImage {
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<crate::models::ApiIcon>>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl ApplicationUnfurlContentJbsImage {
    pub fn new(title: String, url: String) -> ApplicationUnfurlContentJbsImage {
        ApplicationUnfurlContentJbsImage {
            icon: None,
            title,
            url,
        }
    }
}


