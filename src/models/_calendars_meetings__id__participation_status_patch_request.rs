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
pub struct CalendarsMeetingsIdParticipationStatusPatchRequest {
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::EventParticipationStatus,
    #[serde(rename = "targetDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_date: Option<Option<String>>,
    #[serde(rename = "modificationKind", skip_serializing_if = "Option::is_none")]
    pub modification_kind: Option<crate::models::RecurrentModification>,
}

impl CalendarsMeetingsIdParticipationStatusPatchRequest {
    pub fn new(profile_id: String, status: crate::models::EventParticipationStatus) -> CalendarsMeetingsIdParticipationStatusPatchRequest {
        CalendarsMeetingsIdParticipationStatusPatchRequest {
            profile_id,
            status,
            target_date: None,
            modification_kind: None,
        }
    }
}


