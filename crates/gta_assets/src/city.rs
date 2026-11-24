use crate::mesh::{MeshData, Vertex};

pub struct CityGenerator;

impl CityGenerator {
    pub fn build_city_block() -> MeshData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // 1. Asphalt Ground / Road Grid (800x800 plane)
        let ground_color = [0.12, 0.12, 0.14, 1.0]; // Dark asphalt
        let ground_size = 400.0;
        let ground_y = 0.0;

        let base_idx = vertices.len() as u32;
        vertices.push(Vertex {
            position: [-ground_size, ground_y, -ground_size],
            normal: [0.0, 1.0, 0.0],
            uv: [0.0, 0.0],
            color: ground_color,
        });
        vertices.push(Vertex {
            position: [ground_size, ground_y, -ground_size],
            normal: [0.0, 1.0, 0.0],
            uv: [100.0, 0.0],
            color: ground_color,
        });
        vertices.push(Vertex {
            position: [ground_size, ground_y, ground_size],
            normal: [0.0, 1.0, 0.0],
            uv: [100.0, 100.0],
            color: ground_color,
        });
        vertices.push(Vertex {
            position: [-ground_size, ground_y, ground_size],
            normal: [0.0, 1.0, 0.0],
            uv: [0.0, 100.0],
            color: ground_color,
        });

        indices.extend_from_slice(&[
            base_idx, base_idx + 1, base_idx + 2,
            base_idx, base_idx + 2, base_idx + 3,
        ]);

        // 2. Procedural Liberty City style buildings
        // Red brick, weathered concrete, industrial highrises
        let building_coords = [
            // (x, z, width, depth, height, [r, g, b])
            (-35.0, -35.0, 20.0, 20.0, 45.0, [0.38, 0.22, 0.18]), // Classic red brick apartment
            (35.0, -35.0, 22.0, 24.0, 65.0, [0.25, 0.28, 0.32]), // Dark corporate skyscraper
            (-35.0, 35.0, 18.0, 26.0, 38.0, [0.42, 0.36, 0.28]), // Weathered Portland warehouse
            (35.0, 35.0, 24.0, 20.0, 50.0, [0.32, 0.34, 0.36]), // Concrete industrial tower
            (-80.0, 0.0, 25.0, 50.0, 75.0, [0.20, 0.22, 0.26]),  // Monolithic bank high-rise
            (80.0, 0.0, 25.0, 50.0, 60.0, [0.35, 0.25, 0.22]),   // Residential block
        ];

        for &(bx, bz, bw, bd, bh, col) in &building_coords {
            Self::add_box(
                &mut vertices,
                &mut indices,
                [bx, bh * 0.5, bz],
                [bw, bh, bd],
                [col[0], col[1], col[2], 1.0],
            );
        }

        MeshData {
            name: "liberty_city_block".to_string(),
            vertices,
            indices,
        }
    }

    pub fn build_player_mesh() -> MeshData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Classic GTA 3 Claude outfit:
        // Leather black jacket [0.10, 0.10, 0.11]
        // Green cargo pants [0.18, 0.32, 0.18]
        // Skin head [0.82, 0.65, 0.52]

        // Head
        Self::add_box(&mut vertices, &mut indices, [0.0, 1.7, 0.0], [0.35, 0.35, 0.35], [0.82, 0.65, 0.52, 1.0]);
        // Torso (Black leather jacket)
        Self::add_box(&mut vertices, &mut indices, [0.0, 1.15, 0.0], [0.65, 0.75, 0.40], [0.12, 0.12, 0.13, 1.0]);
        // Legs (Green cargo pants)
        Self::add_box(&mut vertices, &mut indices, [0.0, 0.45, 0.0], [0.55, 0.85, 0.35], [0.18, 0.30, 0.16, 1.0]);

        MeshData {
            name: "claude_player".to_string(),
            vertices,
            indices,
        }
    }

    pub fn build_car_mesh() -> MeshData {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Classic Liberty City Kuruma / Sentinel sedan:
        // Body (GTA 3 dark metallic blue [0.12, 0.22, 0.45])
        Self::add_box(&mut vertices, &mut indices, [0.0, 0.45, 0.0], [1.9, 0.60, 4.2], [0.12, 0.22, 0.45, 1.0]);
        // Cabin / Glass roof
        Self::add_box(&mut vertices, &mut indices, [0.0, 0.95, -0.2], [1.6, 0.55, 2.2], [0.15, 0.25, 0.30, 1.0]);
        // Wheels
        Self::add_box(&mut vertices, &mut indices, [-0.95, 0.3, 1.3], [0.25, 0.6, 0.6], [0.05, 0.05, 0.05, 1.0]);
        Self::add_box(&mut vertices, &mut indices, [0.95, 0.3, 1.3], [0.25, 0.6, 0.6], [0.05, 0.05, 0.05, 1.0]);
        Self::add_box(&mut vertices, &mut indices, [-0.95, 0.3, -1.3], [0.25, 0.6, 0.6], [0.05, 0.05, 0.05, 1.0]);
        Self::add_box(&mut vertices, &mut indices, [0.95, 0.3, -1.3], [0.25, 0.6, 0.6], [0.05, 0.05, 0.05, 1.0]);

        MeshData {
            name: "kuruma_sedan".to_string(),
            vertices,
            indices,
        }
    }

    fn add_box(
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        center: [f32; 3],
        size: [f32; 3],
        color: [f32; 4],
    ) {
        let hx = size[0] * 0.5;
        let hy = size[1] * 0.5;
        let hz = size[2] * 0.5;
        let cx = center[0];
        let cy = center[1];
        let cz = center[2];

        // 6 faces * 4 vertices = 24 vertices
        let faces = [
            // Front (+Z)
            ([0.0, 0.0, 1.0], [
                [cx - hx, cy - hy, cz + hz],
                [cx + hx, cy - hy, cz + hz],
                [cx + hx, cy + hy, cz + hz],
                [cx - hx, cy + hy, cz + hz],
            ]),
            // Back (-Z)
            ([0.0, 0.0, -1.0], [
                [cx + hx, cy - hy, cz - hz],
                [cx - hx, cy - hy, cz - hz],
                [cx - hx, cy + hy, cz - hz],
                [cx + hx, cy + hy, cz - hz],
            ]),
            // Top (+Y)
            ([0.0, 1.0, 0.0], [
                [cx - hx, cy + hy, cz + hz],
                [cx + hx, cy + hy, cz + hz],
                [cx + hx, cy + hy, cz - hz],
                [cx - hx, cy + hy, cz - hz],
            ]),
            // Bottom (-Y)
            ([0.0, -1.0, 0.0], [
                [cx - hx, cy - hy, cz - hz],
                [cx + hx, cy - hy, cz - hz],
                [cx + hx, cy - hy, cz + hz],
                [cx - hx, cy - hy, cz + hz],
            ]),
            // Right (+X)
            ([1.0, 0.0, 0.0], [
                [cx + hx, cy - hy, cz + hz],
                [cx + hx, cy - hy, cz - hz],
                [cx + hx, cy + hy, cz - hz],
                [cx + hx, cy + hy, cz + hz],
            ]),
            // Left (-X)
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
                    color,
                });
            }
            indices.extend_from_slice(&[
                base, base + 1, base + 2,
                base, base + 2, base + 3,
            ]);
        }
    }
}
