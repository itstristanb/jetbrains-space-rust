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
pub struct VcsCommitCfParametersInput {
    #[serde(rename = "vcsCFScope")]
    pub vcs_cf_scope: Box<crate::models::VcsCfScopeInputJbsProject>,
}

impl VcsCommitCfParametersInput {
    pub fn new(vcs_cf_scope: crate::models::VcsCfScopeInputJbsProject) -> VcsCommitCfParametersInput {
        VcsCommitCfParametersInput {
            vcs_cf_scope: Box::new(vcs_cf_scope),
        }
    }
}


