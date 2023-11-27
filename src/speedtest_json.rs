use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct SpeedTestJsonResult<'a> {
    #[serde(rename = "Server ID")]
    pub server_id: &'a str,
    #[serde(rename = "Sponsor")]
    pub sponsor: &'a str,
    #[serde(rename = "Server Name")]
    pub server_name: &'a str,
    #[serde(rename = "Timestamp")]
    pub timestamp: &'a str,
    #[serde(rename = "Distance")]
    pub distance: &'a str,
    #[serde(rename = "Ping")]
    pub ping: &'a str,
    #[serde(rename = "Download")]
    pub download: &'a str,
    #[serde(rename = "Upload")]
    pub upload: &'a str,
    #[serde(rename = "Share")]
    pub share: &'a str,
    #[serde(rename = "IP Address")]
    pub ip_address: &'a str,
}
