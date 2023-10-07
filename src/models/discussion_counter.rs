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
pub struct DiscussionCounter {
    #[serde(rename = "resolved")]
    pub resolved: i32,
    #[serde(rename = "unresolved")]
    pub unresolved: i32,
    #[serde(rename = "unresolvedSuggestedEdits")]
    pub unresolved_suggested_edits: i32,
    #[serde(rename = "acceptedSuggestedEdits")]
    pub accepted_suggested_edits: i32,
    #[serde(rename = "rejectedSuggestedEdits")]
    pub rejected_suggested_edits: i32,
}

impl DiscussionCounter {
    pub fn new(resolved: i32, unresolved: i32, unresolved_suggested_edits: i32, accepted_suggested_edits: i32, rejected_suggested_edits: i32) -> DiscussionCounter {
        DiscussionCounter {
            resolved,
            unresolved,
            unresolved_suggested_edits,
            accepted_suggested_edits,
            rejected_suggested_edits,
        }
    }
}


