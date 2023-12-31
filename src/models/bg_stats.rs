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
pub struct BgStats {
    #[serde(rename = "totalBlogs")]
    pub total_blogs: i32,
    #[serde(rename = "teams")]
    pub teams: Vec<crate::models::BgStatsTeamsInner>,
    #[serde(rename = "projects", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Option<Vec<crate::models::BgStatsProjectsInner>>>,
    #[serde(rename = "locations")]
    pub locations: Vec<crate::models::BgStatsLocationsInner>,
}

impl BgStats {
    pub fn new(total_blogs: i32, teams: Vec<crate::models::BgStatsTeamsInner>, locations: Vec<crate::models::BgStatsLocationsInner>) -> BgStats {
        BgStats {
            total_blogs,
            teams,
            projects: None,
            locations,
        }
    }
}


