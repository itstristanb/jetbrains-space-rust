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
pub struct TrackerIssueFieldVisibility {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "systemIssueFieldVisibilities")]
    pub system_issue_field_visibilities: Vec<crate::models::IssueFieldVisibility>,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl TrackerIssueFieldVisibility {
    pub fn new(id: String, system_issue_field_visibilities: Vec<crate::models::IssueFieldVisibility>, archived: bool) -> TrackerIssueFieldVisibility {
        TrackerIssueFieldVisibility {
            id,
            system_issue_field_visibilities,
            archived,
        }
    }
}


