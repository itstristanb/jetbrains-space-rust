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
pub struct MeCodeReviewParticipantRecord {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::CodeReviewParticipantRole>,
    #[serde(rename = "theirTurn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub their_turn: Option<Option<bool>>,
    #[serde(rename = "reviewerState", skip_serializing_if = "Option::is_none")]
    pub reviewer_state: Option<crate::models::ReviewerState>,
    #[serde(rename = "isApproveSticky", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_approve_sticky: Option<Option<bool>>,
    #[serde(rename = "reviewOnlyOwnedFiles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub review_only_owned_files: Option<Option<bool>>,
    #[serde(rename = "review")]
    pub review: Box<crate::models::CodeReviewRecord>,
    #[serde(rename = "participants")]
    pub participants: Box<crate::models::CodeReviewParticipants>,
    #[serde(rename = "pendingCounter")]
    pub pending_counter: Box<crate::models::CodeReviewPendingMessageCounter>,
    #[serde(rename = "archived")]
    pub archived: bool,
}

impl MeCodeReviewParticipantRecord {
    pub fn new(id: String, review: crate::models::CodeReviewRecord, participants: crate::models::CodeReviewParticipants, pending_counter: crate::models::CodeReviewPendingMessageCounter, archived: bool) -> MeCodeReviewParticipantRecord {
        MeCodeReviewParticipantRecord {
            id,
            role: None,
            their_turn: None,
            reviewer_state: None,
            is_approve_sticky: None,
            review_only_owned_files: None,
            review: Box::new(review),
            participants: Box::new(participants),
            pending_counter: Box::new(pending_counter),
            archived,
        }
    }
}

