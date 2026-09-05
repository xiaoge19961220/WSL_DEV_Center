use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Distro {
    pub name: String,
    pub state: String,
    pub version: Option<u8>,
    pub is_default: bool,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OnlineDistro {
    pub name: String,
    pub friendly_name: String,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub success: bool,
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub distro: String,
    pub os_version_text: Option<String>,
    pub kernel_version_text: Option<String>,
    pub cpu_text: Option<String>,
    pub memory_text: Option<String>,
    pub disk_text: Option<String>,
    pub uptime_text: Option<String>,
    pub process_count: Option<u32>,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    pub protocol: String,
    pub local_address: String,
    pub port: u16,
    pub process_name: Option<String>,
    pub pid: Option<u32>,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Docker {
    #[serde(rename(deserialize = "ID"))]
    pub id: String,
    #[serde(rename(deserialize = "Image"))]
    pub image: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "Ports"))]
    pub ports: Option<String>,
    #[serde(rename(deserialize = "Names"))]
    pub names: String,
    #[serde(rename(deserialize = "Command"))]
    pub command: Option<String>,
    #[serde(rename(deserialize = "CreatedAt"))]
    pub created: Option<String>,
}
