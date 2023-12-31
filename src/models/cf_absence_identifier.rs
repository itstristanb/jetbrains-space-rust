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
pub struct CfAbsenceIdentifier {
    #[serde(rename = "absence")]
    pub absence: Box<crate::models::AbsenceIdentifierJbsId>,
}

impl CfAbsenceIdentifier {
    pub fn new(absence: crate::models::AbsenceIdentifierJbsId) -> CfAbsenceIdentifier {
        CfAbsenceIdentifier {
            absence: Box::new(absence),
        }
    }
}


