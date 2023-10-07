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
pub enum IssueListGrouping {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "TAG")]
    Tag,
    #[serde(rename = "ASSIGNEE")]
    Assignee,
    #[serde(rename = "STATUS")]
    Status,
    #[serde(rename = "PROJECT")]
    Project,

}

impl ToString for IssueListGrouping {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("NONE"),
            Self::Tag => String::from("TAG"),
            Self::Assignee => String::from("ASSIGNEE"),
            Self::Status => String::from("STATUS"),
            Self::Project => String::from("PROJECT"),
        }
    }
}

impl Default for IssueListGrouping {
    fn default() -> IssueListGrouping {
        Self::None
    }
}




