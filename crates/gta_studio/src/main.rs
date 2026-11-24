use std::env;
use std::fs;
use std::path::Path;
use gta_assets::{AssetCompiler, GtaAssetSpec, TextureGenerator};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "gen-textures" => {
            println!("Generating authentic GTA 3 pixel textures in assets/textures/...");
            let tex_dir = Path::new("assets/textures");
            let _ = fs::create_dir_all(tex_dir);
            TextureGenerator::generate_brick_texture(tex_dir.join("brick_wall.png"));
            TextureGenerator::generate_road_texture(tex_dir.join("asphalt_road.png"));
            TextureGenerator::generate_taxi_texture(tex_dir.join("taxi_livery.png"));
            TextureGenerator::generate_mafia_suit_texture(tex_dir.join("mafia_suit.png"));
            println!("All textures generated successfully (brick, road, taxi, mafia_suit)!");
        }
        "compile" => {
            if args.len() < 3 {
                eprintln!("Usage: gta_studio compile <file.json>");
                return;
            }
            let path = &args[2];
            let data = fs::read_to_string(path).expect("Read asset file");
            let spec: GtaAssetSpec = serde_json::from_str(&data).expect("Parse asset JSON");
            let mesh = AssetCompiler::compile_to_mesh(&spec);
            println!(
                "Asset '{}' compiled! Vertices: {}, Triangles: {}",
                mesh.name,
                mesh.vertices.len(),
                mesh.indices.len() / 3
            );
        }
        "list" => {
            list_assets();
        }
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("GTA 3 Asset Studio - Professional Pipeline");
    println!("Commands:");
    println!("  gta_studio gen-textures             - Synthesize authentic 256x256 GTA 3 texture atlases (PNG)");
    println!("  gta_studio compile <file.json>      - Validate and compile asset into game format");
    println!("  gta_studio list                     - List all available models and textures");
}

fn list_assets() {
    let dir = Path::new("assets");
    if let Ok(entries) = fs::read_dir(dir) {
        println!("Available Model Assets:");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json")
                || path.extension().and_then(|e| e.to_str()) == Some("obj")
            {
                println!(" - {}", path.file_name().unwrap().to_string_lossy());
            }
        }
    }
    let tex_dir = Path::new("assets/textures");
    if let Ok(entries) = fs::read_dir(tex_dir) {
        println!("Available Textures:");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("png") {
                println!(" - [PNG] {}", path.file_name().unwrap().to_string_lossy());
            }
        }
    }
}
