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
pub struct DtoMeetingRsvp {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "member")]
    pub member: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "meeting")]
    pub meeting: Box<crate::models::DtoMeeting>,
    #[serde(rename = "status")]
    pub status: crate::models::EventParticipationStatus,
}

impl DtoMeetingRsvp {
    pub fn new(id: String, archived: bool, member: crate::models::TdMemberProfile, meeting: crate::models::DtoMeeting, status: crate::models::EventParticipationStatus) -> DtoMeetingRsvp {
        DtoMeetingRsvp {
            id,
            archived,
            member: Box::new(member),
            meeting: Box::new(meeting),
            status,
        }
    }
}


