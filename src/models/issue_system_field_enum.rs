/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IssueSystemFieldEnum {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ASSIGNEE")]
    Assignee,
    #[serde(rename = "CREATED_BY")]
    CreatedBy,
    #[serde(rename = "STATUS")]
    Status,
    #[serde(rename = "TAG")]
    Tag,
    #[serde(rename = "CREATION_TIME")]
    CreationTime,
    #[serde(rename = "DUE_DATE")]
    DueDate,
    #[serde(rename = "TITLE")]
    Title,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "PARENT_ISSUES")]
    ParentIssues,
    #[serde(rename = "CHECKLISTS")]
    Checklists,
    #[serde(rename = "BOARD")]
    Board,
    #[serde(rename = "SUBSCRIBER")]
    Subscriber,
    #[serde(rename = "IMPORT_TRANSACTION")]
    ImportTransaction,

}

impl ToString for IssueSystemFieldEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Project => String::from("PROJECT"),
            Self::Assignee => String::from("ASSIGNEE"),
            Self::CreatedBy => String::from("CREATED_BY"),
            Self::Status => String::from("STATUS"),
            Self::Tag => String::from("TAG"),
            Self::CreationTime => String::from("CREATION_TIME"),
            Self::DueDate => String::from("DUE_DATE"),
            Self::Title => String::from("TITLE"),
            Self::Deleted => String::from("DELETED"),
            Self::ParentIssues => String::from("PARENT_ISSUES"),
            Self::Checklists => String::from("CHECKLISTS"),
            Self::Board => String::from("BOARD"),
            Self::Subscriber => String::from("SUBSCRIBER"),
            Self::ImportTransaction => String::from("IMPORT_TRANSACTION"),
        }
    }
}

impl Default for IssueSystemFieldEnum {
    fn default() -> IssueSystemFieldEnum {
        Self::Project
    }
}




