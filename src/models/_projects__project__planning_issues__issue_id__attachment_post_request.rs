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
pub struct ProjectsProjectPlanningIssuesIssueIdAttachmentPostRequest {
    #[serde(rename = "attachment")]
    pub attachment: Box<crate::models::AttachmentIn>,
}

impl ProjectsProjectPlanningIssuesIssueIdAttachmentPostRequest {
    pub fn new(attachment: crate::models::AttachmentIn) -> ProjectsProjectPlanningIssuesIssueIdAttachmentPostRequest {
        ProjectsProjectPlanningIssuesIssueIdAttachmentPostRequest {
            attachment: Box::new(attachment),
        }
    }
}


