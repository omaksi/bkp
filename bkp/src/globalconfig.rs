pub struct GlobalConfig {
    pub local_storage_location: String,
    pub remote_storage_address: String,
    pub remote_storage_access_id: String,
    pub remote_storage_secret_key: String,
}

pub fn get_global_config() -> GlobalConfig {
    GlobalConfig {
        local_storage_location: String::from(
            "/Users/ondrej/Documents/GitHub/bkp/bkp/testdata/storage",
        ),
        remote_storage_address: String::from("http://localhost:9000"),
        remote_storage_access_id: String::from("minioadmin"),
        remote_storage_secret_key: String::from("minioadmin"),
    }
}
