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
pub struct RtLinkAttrs {
    #[serde(rename = "href")]
    pub href: String,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "mention", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mention: Option<Option<String>>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<crate::models::RtLinkDetails>>,
}

impl RtLinkAttrs {
    pub fn new(href: String) -> RtLinkAttrs {
        RtLinkAttrs {
            href,
            title: None,
            mention: None,
            details: None,
        }
    }
}


