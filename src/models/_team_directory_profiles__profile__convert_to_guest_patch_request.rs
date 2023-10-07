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
pub struct TeamDirectoryProfilesProfileConvertToGuestPatchRequest {
    #[serde(rename = "dryrun")]
    pub dryrun: bool,
    #[serde(rename = "guestType", skip_serializing_if = "Option::is_none")]
    pub guest_type: Option<Box<crate::models::GuestType>>,
}

impl TeamDirectoryProfilesProfileConvertToGuestPatchRequest {
    pub fn new(dryrun: bool) -> TeamDirectoryProfilesProfileConvertToGuestPatchRequest {
        TeamDirectoryProfilesProfileConvertToGuestPatchRequest {
            dryrun,
            guest_type: None,
        }
    }
}

