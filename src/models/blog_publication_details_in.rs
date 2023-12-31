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
pub struct BlogPublicationDetailsIn {
    #[serde(rename = "teams", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Option<Vec<String>>>,
    #[serde(rename = "locations", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Option<Vec<String>>>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<crate::models::CalendarEventIn>>,
    #[serde(rename = "article", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub article: Option<Option<String>>,
}

impl BlogPublicationDetailsIn {
    pub fn new() -> BlogPublicationDetailsIn {
        BlogPublicationDetailsIn {
            teams: None,
            locations: None,
            event: None,
            article: None,
        }
    }
}


