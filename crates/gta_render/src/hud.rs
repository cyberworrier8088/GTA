use gta_assets::Vertex;

pub struct HudRenderer;

impl HudRenderer {
    /// Generates 2D screen overlay quad vertices for classic GTA 3 HUD:
    /// - Radar Minimap ring (bottom left)
    /// - Health bar (bright green)
    /// - Armor bar (bright blue)
    /// - Weapon / Wanted Stars placeholder
    pub fn build_hud_geometry(aspect: f32, health: f32, armor: f32) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Screen space coordinates: X [-aspect, aspect], Y [-1.0, 1.0]
        
        // 1. Radar Minimap outer ring and inner dark display (bottom left)
        let radar_cx = -aspect + 0.35;
        let radar_cy = -0.65;
        let radar_r = 0.22;

        // Dark radar background
        Self::add_circle_quad(&mut vertices, &mut indices, radar_cx, radar_cy, radar_r, [0.08, 0.12, 0.15, 0.85]);
        // Radar green blip (player indicator in center)
        Self::add_quad(&mut vertices, &mut indices, radar_cx - 0.015, radar_cy - 0.015, 0.03, 0.03, [0.2, 0.9, 0.3, 1.0]);

        // 2. Health Bar (top right, classic bright green #33CC33)
        let bar_width = 0.35;
        let bar_height = 0.035;
        let top_right_x = aspect - 0.45;
        let health_y = 0.85;

        // Health bar background (dark red/black)
        Self::add_quad(&mut vertices, &mut indices, top_right_x, health_y, bar_width, bar_height, [0.2, 0.05, 0.05, 0.8]);
        // Health bar foreground (proportional to health)
        let health_fill = (health / 100.0).clamp(0.0, 1.0) * bar_width;
        Self::add_quad(&mut vertices, &mut indices, top_right_x, health_y, health_fill, bar_height, [0.2, 0.85, 0.25, 1.0]);

        // 3. Armor Bar (top right, classic bright cyan/blue)
        let armor_y = 0.80;
        Self::add_quad(&mut vertices, &mut indices, top_right_x, armor_y, bar_width, bar_height, [0.05, 0.1, 0.2, 0.8]);
        let armor_fill = (armor / 100.0).clamp(0.0, 1.0) * bar_width;
        Self::add_quad(&mut vertices, &mut indices, top_right_x, armor_y, armor_fill, bar_height, [0.2, 0.6, 0.95, 1.0]);

        // 4. Money / Weapon slot display box
        let weapon_slot_x = aspect - 0.20;
        let weapon_slot_y = 0.65;
        Self::add_quad(&mut vertices, &mut indices, weapon_slot_x, weapon_slot_y, 0.12, 0.10, [0.15, 0.18, 0.20, 0.75]);

        (vertices, indices)
    }

    fn add_quad(
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
    ) {
        let base = vertices.len() as u32;
        vertices.push(Vertex { position: [x, y, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 0.0], color });
        vertices.push(Vertex { position: [x + w, y, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 0.0], color });
        vertices.push(Vertex { position: [x + w, y + h, 0.0], normal: [0.0, 0.0, 1.0], uv: [1.0, 1.0], color });
        vertices.push(Vertex { position: [x, y + h, 0.0], normal: [0.0, 0.0, 1.0], uv: [0.0, 1.0], color });

        indices.extend_from_slice(&[
            base, base + 1, base + 2,
            base, base + 2, base + 3,
        ]);
    }

    fn add_circle_quad(
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        cx: f32,
        cy: f32,
        r: f32,
        color: [f32; 4],
    ) {
        Self::add_quad(vertices, indices, cx - r, cy - r, r * 2.0, r * 2.0, color);
    }
}
