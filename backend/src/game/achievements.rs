pub struct AchievementManager {
    app_id: u32
}

impl AchievementManager {
    pub fn new(app_id: u32) -> AchievementManager {
        AchievementManager {
            app_id: app_id
        }
    }
}