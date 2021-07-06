use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct DeviceRegistrationRequest {
    pub capabilities: HashMap<String, bool>,
    pub fetchesMessages: bool,
    pub name: String,
    pub registrationId: u32,
    pub supportsSms: bool,
    pub unidentifiedAccessKey: Option<String>,
    pub unrestrictedUnidentifiedAccess: bool,
}

impl DeviceRegistrationRequest {
    pub fn new(name: String, registration_id: u32) -> Self {
        let mut capabilities = HashMap::new();
        capabilities.insert("gv2-3".to_string(), true);
        capabilities.insert("gv1-migration".to_string(), true);
        capabilities.insert("senderKey".to_string(), false);
        Self {
            capabilities,
            fetchesMessages: true,
            name,
            registrationId: registration_id,
            supportsSms: false,
            unidentifiedAccessKey: None,
            unrestrictedUnidentifiedAccess: false,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DeviceRegistrationResponse {
    pub uuid: Option<String>,
    pub deviceId: Option<u32>,
}
