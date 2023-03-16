use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub difficulty: String,
    pub id: usize,
    pub is_complete: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Achievements {
    pub collection: Vec<Achievement>,
    pub len: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VeryHardAchievements {
    pub collection: Vec<Achievement>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HardAchievements {
    pub collection: Vec<Achievement>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediumAchievements {
    pub collection: Vec<Achievement>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EasyAchievements {
    pub collection: Vec<Achievement>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VeryEasyAchievements {
    pub collection: Vec<Achievement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    #[serde(default)]
    pub current: Option<Achievement>,
    pub very_hard: VeryHardAchievements,
    pub hard: HardAchievements,
    pub medium: MediumAchievements,
    pub easy: EasyAchievements,
    pub very_easy: VeryEasyAchievements,
}
