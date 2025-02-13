use rand::Rng;

pub struct CheatSettings {
    pub aimbot_enabled: bool,
    pub esp_enabled: bool,
    pub no_recoil_enabled: bool,
    pub speed_multiplier: f32,
}

impl CheatSettings {
    pub fn new() -> Self {
        CheatSettings {
            aimbot_enabled: false,
            esp_enabled: false,
            no_recoil_enabled: false,
            speed_multiplier: 1.0,
        }
    }

    pub fn toggle_aimbot(&mut self) {
        self.aimbot_enabled = !self.aimbot_enabled;
    }

    pub fn toggle_esp(&mut self) {
        self.esp_enabled = !self.esp_enabled;
    }

    pub fn toggle_no_recoil(&mut self) {
        self.no_recoil_enabled = !self.no_recoil_enabled;
    }

    pub fn set_speed_multiplier(&mut self, multiplier: f32) {
        self.speed_multiplier = multiplier;
    }
}