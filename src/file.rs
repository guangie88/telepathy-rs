#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<u64>,
    pub file_path: Option<String>,
}
