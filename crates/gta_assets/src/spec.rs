use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartSpec {
    pub name: String,
    pub shape: String, // "box", "cylinder", "wedge"
    pub center: [f32; 3],
    pub size: [f32; 3],
    pub color: [f32; 4],
    #[serde(default)]
    pub uv_scale: [f32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GtaAssetSpec {
    pub name: String,
    pub category: String, // "vehicle", "building", "character", "prop"
    pub parts: Vec<PartSpec>,
    #[serde(default)]
    pub collision_box: Option<[f32; 3]>,
}
