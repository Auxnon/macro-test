pub struct Global {
    pub DEVELOPER_MODE: bool,
}

impl Default for Global {
    fn default() -> Global {
        Global {
            DEVELOPER_MODE: false,
        }
    }
}
