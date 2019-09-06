pub struct AppConfig {
    CenterHost: String,
    CenterPort: u16,
    RoomHost: String,
    RoomPort: u16,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            CenterHost: String::new(),
            CenterPort: 0,
            RoomHost: String::new(),
            RoomPort: 0,
        }
    }

    pub fn with_config_file(config_name: &str) -> AppConfig {
        AppConfig {
            CenterHost: String::new(),
            CenterPort: 0,
            RoomHost: String::new(),
            RoomPort: 0,
        }
    }

}
