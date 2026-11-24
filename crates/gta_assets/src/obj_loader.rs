use std::path::Path;
use crate::mesh::{MeshData, Vertex};

pub struct ObjLoader;

impl ObjLoader {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<MeshData, String> {
        let (models, _materials) = tobj::load_obj(
            path.as_ref(),
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )
        .map_err(|e| format!("Failed to load OBJ: {}", e))?;

        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let base_idx = all_vertices.len() as u32;

            let num_vertices = mesh.positions.len() / 3;
            for i in 0..num_vertices {
                let pos = [
                    mesh.positions[i * 3],
                    mesh.positions[i * 3 + 1],
                    mesh.positions[i * 3 + 2],
                ];

                let normal = if !mesh.normals.is_empty() && (i * 3 + 2) < mesh.normals.len() {
                    [mesh.normals[i * 3], mesh.normals[i * 3 + 1], mesh.normals[i * 3 + 2]]
                } else {
                    [0.0, 1.0, 0.0]
                };

                let uv = if !mesh.texcoords.is_empty() && (i * 2 + 1) < mesh.texcoords.len() {
                    [mesh.texcoords[i * 2], 1.0 - mesh.texcoords[i * 2 + 1]] // Flip V for standard DirectX/Vulkan
                } else {
                    [0.0, 0.0]
                };

                all_vertices.push(Vertex {
                    position: pos,
                    normal,
                    uv,
                    color: [1.0, 1.0, 1.0, 1.0],
                });
            }

            for idx in &mesh.indices {
                all_indices.push(base_idx + idx);
            }
        }

        Ok(MeshData {
            name: path.as_ref().file_stem().unwrap().to_string_lossy().to_string(),
            vertices: all_vertices,
            indices: all_indices,
        })
    }
}
