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
pub struct UnfurlDetailsIssueImportTransaction {
    #[serde(rename = "importTransaction")]
    pub import_transaction: Box<crate::models::ImportTransactionRecord>,
}

impl UnfurlDetailsIssueImportTransaction {
    pub fn new(import_transaction: crate::models::ImportTransactionRecord) -> UnfurlDetailsIssueImportTransaction {
        UnfurlDetailsIssueImportTransaction {
            import_transaction: Box::new(import_transaction),
        }
    }
}

