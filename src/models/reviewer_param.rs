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
pub struct ReviewerParam {
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "isCodeOwner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_code_owner: Option<Option<bool>>,
    #[serde(rename = "qualityGateSlot", skip_serializing_if = "Option::is_none")]
    pub quality_gate_slot: Option<Box<crate::models::CodeReviewParticipantSlotBase>>,
}

impl ReviewerParam {
    pub fn new(profile_id: String) -> ReviewerParam {
        ReviewerParam {
            profile_id,
            is_code_owner: None,
            quality_gate_slot: None,
        }
    }
}

