use std::env;
use std::fs;
use std::path::Path;
use gta_assets::{AssetCompiler, GtaAssetSpec, PartSpec};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 4 {
                eprintln!("Usage: gta_studio new <category> <name>");
                eprintln!("Categories: vehicle, building, character, prop");
                return;
            }
            let category = &args[2];
            let name = &args[3];
            create_template(category, name);
        }
        "compile" => {
            if args.len() < 3 {
                eprintln!("Usage: gta_studio compile <path-to-json>");
                return;
            }
            let path = &args[2];
            compile_asset(path);
        }
        "list" => {
            list_assets();
        }
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("GTA 3 Asset Studio CLI");
    println!("Commands:");
    println!("  gta_studio new <vehicle|building|character|prop> <name>  - Create a new low-poly asset template");
    println!("  gta_studio compile <file.json>                           - Validate and compile asset into game directory");
    println!("  gta_studio list                                          - List all available assets in assets/ directory");
}

fn create_template(category: &str, name: &str) {
    let spec = match category {
        "vehicle" => GtaAssetSpec {
            name: name.to_string(),
            category: "vehicle".to_string(),
            parts: vec![
                PartSpec {
                    name: "chassis".to_string(),
                    shape: "box".to_string(),
                    center: [0.0, 0.45, 0.0],
                    size: [1.9, 0.60, 4.2],
                    color: [0.15, 0.45, 0.20, 1.0], // Dark green
                    uv_scale: [1.0, 1.0],
                },
                PartSpec {
                    name: "cabin".to_string(),
                    shape: "box".to_string(),
                    center: [0.0, 0.95, -0.2],
                    size: [1.6, 0.55, 2.2],
                    color: [0.15, 0.25, 0.30, 1.0], // Glass tint
                    uv_scale: [1.0, 1.0],
                },
            ],
            collision_box: Some([1.9, 1.4, 4.2]),
        },
        "character" => GtaAssetSpec {
            name: name.to_string(),
            category: "character".to_string(),
            parts: vec![
                PartSpec {
                    name: "head".to_string(),
                    shape: "box".to_string(),
                    center: [0.0, 1.7, 0.0],
                    size: [0.35, 0.35, 0.35],
                    color: [0.82, 0.65, 0.52, 1.0],
                    uv_scale: [1.0, 1.0],
                },
                PartSpec {
                    name: "jacket".to_string(),
                    shape: "box".to_string(),
                    center: [0.0, 1.15, 0.0],
                    size: [0.65, 0.75, 0.40],
                    color: [0.15, 0.15, 0.18, 1.0],
                    uv_scale: [1.0, 1.0],
                },
                PartSpec {
                    name: "pants".to_string(),
                    shape: "box".to_string(),
                    center: [0.0, 0.45, 0.0],
                    size: [0.55, 0.85, 0.35],
                    color: [0.20, 0.25, 0.35, 1.0],
                    uv_scale: [1.0, 1.0],
                },
            ],
            collision_box: Some([0.7, 1.8, 0.4]),
        },
        _ => GtaAssetSpec {
            name: name.to_string(),
            category: category.to_string(),
            parts: vec![
                PartSpec {
                    name: "base".to_string(),
                    shape: "box".to_string(),
                    center: [0.0, 10.0, 0.0],
                    size: [20.0, 20.0, 20.0],
                    color: [0.38, 0.22, 0.18, 1.0],
                    uv_scale: [1.0, 1.0],
                },
            ],
            collision_box: Some([20.0, 20.0, 20.0]),
        },
    };

    let dir = Path::new("assets");
    if !dir.exists() {
        let _ = fs::create_dir_all(dir);
    }

    let file_path = dir.join(format!("{}.json", name));
    let json_text = serde_json::to_string_pretty(&spec).expect("Serialize asset JSON");
    fs::write(&file_path, json_text).expect("Write asset JSON file");
    println!("Asset template created: {}", file_path.display());
}

fn compile_asset(path: &str) {
    let data = fs::read_to_string(path).expect("Read asset file");
    let spec: GtaAssetSpec = serde_json::from_str(&data).expect("Parse asset JSON");
    let mesh = AssetCompiler::compile_to_mesh(&spec);
    println!(
        "Asset compiled successfully! Name: '{}', Vertices: {}, Indices: {}",
        mesh.name,
        mesh.vertices.len(),
        mesh.indices.len()
    );
}

fn list_assets() {
    let dir = Path::new("assets");
    if !dir.exists() {
        println!("No assets/ folder found.");
        return;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        println!("Available Assets:");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                println!(" - {}", path.file_name().unwrap().to_string_lossy());
            }
        }
    }
}
