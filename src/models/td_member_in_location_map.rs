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
pub struct TdMemberInLocationMap {
    #[serde(rename = "profile")]
    pub profile: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "memberLocation")]
    pub member_location: Box<crate::models::TdMemberLocation>,
    #[serde(rename = "locationMapPoint", skip_serializing_if = "Option::is_none")]
    pub location_map_point: Option<Box<crate::models::TdLocationMapPoint>>,
}

impl TdMemberInLocationMap {
    pub fn new(profile: crate::models::TdMemberProfile, member_location: crate::models::TdMemberLocation) -> TdMemberInLocationMap {
        TdMemberInLocationMap {
            profile: Box::new(profile),
            member_location: Box::new(member_location),
            location_map_point: None,
        }
    }
}


