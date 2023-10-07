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
pub struct GitRepositorySettingsJbsQualityGateApproval {
    #[serde(rename = "minApprovals")]
    pub min_approvals: i32,
    #[serde(rename = "approvedBy")]
    pub approved_by: Vec<String>,
}

impl GitRepositorySettingsJbsQualityGateApproval {
    pub fn new(min_approvals: i32, approved_by: Vec<String>) -> GitRepositorySettingsJbsQualityGateApproval {
        GitRepositorySettingsJbsQualityGateApproval {
            min_approvals,
            approved_by,
        }
    }
}


