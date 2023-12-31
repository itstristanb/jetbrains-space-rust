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
pub struct BlogPublicationDetails {
    #[serde(rename = "teams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Option<Vec<crate::models::TdTeam>>>,
    #[serde(rename = "locations", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Option<Vec<crate::models::TdLocation>>>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<crate::models::CalendarEvent>>,
    #[serde(rename = "article", skip_serializing_if = "Option::is_none")]
    pub article: Option<Box<crate::models::ArticleRecord>>,
}

impl BlogPublicationDetails {
    pub fn new() -> BlogPublicationDetails {
        BlogPublicationDetails {
            teams: None,
            locations: None,
            event: None,
            article: None,
        }
    }
}


