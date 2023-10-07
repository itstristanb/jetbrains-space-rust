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
pub struct DslEvaluationConfig {
    #[serde(rename = "experimentalOptIns")]
    pub experimental_opt_ins: Vec<String>,
    #[serde(rename = "runtimeInfo")]
    pub runtime_info: Box<crate::models::DslRuntimeInfo>,
}

impl DslEvaluationConfig {
    pub fn new(experimental_opt_ins: Vec<String>, runtime_info: crate::models::DslRuntimeInfo) -> DslEvaluationConfig {
        DslEvaluationConfig {
            experimental_opt_ins,
            runtime_info: Box::new(runtime_info),
        }
    }
}


