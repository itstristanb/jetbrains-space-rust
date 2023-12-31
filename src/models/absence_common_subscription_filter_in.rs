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
pub struct AbsenceCommonSubscriptionFilterIn {
    #[serde(rename = "reasons")]
    pub reasons: Vec<String>,
}

impl AbsenceCommonSubscriptionFilterIn {
    pub fn new(reasons: Vec<String>) -> AbsenceCommonSubscriptionFilterIn {
        AbsenceCommonSubscriptionFilterIn {
            reasons,
        }
    }
}


