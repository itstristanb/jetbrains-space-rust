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
pub struct UnfurlDetailsSnapshotDiff {
    #[serde(rename = "snapshotId")]
    pub snapshot_id: String,
    #[serde(rename = "baseSnapshotId")]
    pub base_snapshot_id: String,
}

impl UnfurlDetailsSnapshotDiff {
    pub fn new(snapshot_id: String, base_snapshot_id: String) -> UnfurlDetailsSnapshotDiff {
        UnfurlDetailsSnapshotDiff {
            snapshot_id,
            base_snapshot_id,
        }
    }
}


