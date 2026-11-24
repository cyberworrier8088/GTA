use std::fs;
use std::path::Path;
use crate::compiler::AssetCompiler;
use crate::mesh::MeshData;
use crate::spec::GtaAssetSpec;

pub struct AssetManager;

impl AssetManager {
    pub fn load_asset_file<P: AsRef<Path>>(path: P) -> Result<MeshData, String> {
        let text = fs::read_to_string(path.as_ref())
            .map_err(|e| format!("Failed to read asset {}: {}", path.as_ref().display(), e))?;
        let spec: GtaAssetSpec = serde_json::from_str(&text)
            .map_err(|e| format!("Failed to parse JSON for {}: {}", path.as_ref().display(), e))?;
        Ok(AssetCompiler::compile_to_mesh(&spec))
    }
}
