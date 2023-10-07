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
pub struct AbsenceApproval {
    #[serde(rename = "approved")]
    pub approved: bool,
    #[serde(rename = "approvedBy")]
    pub approved_by: Box<crate::models::TdMemberProfile>,
    #[serde(rename = "approvedAt")]
    pub approved_at: String,
}

impl AbsenceApproval {
    pub fn new(approved: bool, approved_by: crate::models::TdMemberProfile, approved_at: String) -> AbsenceApproval {
        AbsenceApproval {
            approved,
            approved_by: Box::new(approved_by),
            approved_at,
        }
    }
}

