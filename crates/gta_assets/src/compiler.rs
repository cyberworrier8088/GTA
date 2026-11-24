use crate::mesh::{MeshData, Vertex};
use crate::spec::{GtaAssetSpec, PartSpec};

pub struct AssetCompiler;

impl AssetCompiler {
    pub fn compile_to_mesh(spec: &GtaAssetSpec) -> MeshData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for part in &spec.parts {
            match part.shape.as_str() {
                "box" => Self::add_box_part(&mut vertices, &mut indices, part),
                _ => Self::add_box_part(&mut vertices, &mut indices, part),
            }
        }

        MeshData {
            name: spec.name.clone(),
            vertices,
            indices,
        }
    }

    fn add_box_part(
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        part: &PartSpec,
    ) {
        let hx = part.size[0] * 0.5;
        let hy = part.size[1] * 0.5;
        let hz = part.size[2] * 0.5;
        let cx = part.center[0];
        let cy = part.center[1];
        let cz = part.center[2];

        let faces = [
            ([0.0, 0.0, 1.0], [
                [cx - hx, cy - hy, cz + hz],
                [cx + hx, cy - hy, cz + hz],
                [cx + hx, cy + hy, cz + hz],
                [cx - hx, cy + hy, cz + hz],
            ]),
            ([0.0, 0.0, -1.0], [
                [cx + hx, cy - hy, cz - hz],
                [cx - hx, cy - hy, cz - hz],
                [cx - hx, cy + hy, cz - hz],
                [cx + hx, cy + hy, cz - hz],
            ]),
            ([0.0, 1.0, 0.0], [
                [cx - hx, cy + hy, cz + hz],
                [cx + hx, cy + hy, cz + hz],
                [cx + hx, cy + hy, cz - hz],
                [cx - hx, cy + hy, cz - hz],
            ]),
            ([0.0, -1.0, 0.0], [
                [cx - hx, cy - hy, cz - hz],
                [cx + hx, cy - hy, cz - hz],
                [cx + hx, cy - hy, cz + hz],
                [cx - hx, cy - hy, cz + hz],
            ]),
            ([1.0, 0.0, 0.0], [
                [cx + hx, cy - hy, cz + hz],
                [cx + hx, cy - hy, cz - hz],
                [cx + hx, cy + hy, cz - hz],
                [cx + hx, cy + hy, cz + hz],
            ]),
            ([-1.0, 0.0, 0.0], [
                [cx - hx, cy - hy, cz - hz],
                [cx - hx, cy - hy, cz + hz],
                [cx - hx, cy + hy, cz + hz],
                [cx - hx, cy + hy, cz - hz],
            ]),
        ];

        for (normal, corners) in faces {
            let base = vertices.len() as u32;
            for corner in corners {
                vertices.push(Vertex {
                    position: corner,
                    normal,
                    uv: [0.0, 0.0],
                    color: part.color,
                });
            }
            indices.extend_from_slice(&[
                base, base + 1, base + 2,
                base, base + 2, base + 3,
            ]);
        }
    }
}
