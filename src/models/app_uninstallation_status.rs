/*
 * Space
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023.3.0-DEV.171131
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppUninstallationStatus {
    #[serde(rename = "SENDING_UNINSTALL_PAYLOAD")]
    SendingUninstallPayload,
    #[serde(rename = "RETRYING_SENDING_UNINSTALL_PAYLOAD")]
    RetryingSendingUninstallPayload,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "UNINSTALLED")]
    Uninstalled,

}

impl ToString for AppUninstallationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::SendingUninstallPayload => String::from("SENDING_UNINSTALL_PAYLOAD"),
            Self::RetryingSendingUninstallPayload => String::from("RETRYING_SENDING_UNINSTALL_PAYLOAD"),
            Self::Failed => String::from("FAILED"),
            Self::Uninstalled => String::from("UNINSTALLED"),
        }
    }
}

impl Default for AppUninstallationStatus {
    fn default() -> AppUninstallationStatus {
        Self::SendingUninstallPayload
    }
}




