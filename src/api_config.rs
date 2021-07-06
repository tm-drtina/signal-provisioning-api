pub struct ApiConfig {
    pub http_protocol: String,
    pub socket_protocol: String,
    pub host: String,
    pub port: u16,
    pub provisioning_socket_path: String,
    pub devices_path: String,
    pub cert_bytes: Box<[u8]>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        let cert_bytes = &include_bytes!("./signal_certs.pem")[..];

        Self {
            http_protocol: "https://".to_string(),
            socket_protocol: "wss://".to_string(),
            host: "textsecure-service.whispersystems.org".to_string(),
            port: 443,
            provisioning_socket_path: "/v1/websocket/provisioning/".to_string(),
            devices_path: "/v1/devices/".to_string(),
            cert_bytes: Vec::from(cert_bytes).into_boxed_slice(),
        }
    }
}
