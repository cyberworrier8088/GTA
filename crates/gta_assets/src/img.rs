// GTA 3 .IMG and .DIR archive parser
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Clone)]
pub struct ImgEntry {
    pub name: String,
    pub offset: u64,
    pub size: usize,
}

pub struct ImgArchive {
    pub entries: HashMap<String, ImgEntry>,
}

impl ImgArchive {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn parse_dir<R: Read>(mut reader: R) -> std::io::Result<Self> {
        let mut entries = HashMap::new();
        let mut buf = [0u8; 32];
        while reader.read_exact(&mut buf).is_ok() {
            let offset_blocks = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as u64;
            let size_blocks = u32::from_le_bytes(buf[4..8].try_into().unwrap()) as usize;
            let name_bytes = &buf[8..32];
            let name_len = name_bytes.iter().position(|&c| c == 0).unwrap_or(24);
            let name = String::from_utf8_lossy(&name_bytes[..name_len]).to_lowercase();

            entries.insert(
                name.clone(),
                ImgEntry {
                    name,
                    offset: offset_blocks * 2048,
                    size: size_blocks * 2048,
                },
            );
        }
        Ok(Self { entries })
    }

    pub fn read_file<R: Read + Seek>(&self, reader: &mut R, name: &str) -> Option<Vec<u8>> {
        let entry = self.entries.get(&name.to_lowercase())?;
        reader.seek(SeekFrom::Start(entry.offset)).ok()?;
        let mut data = vec![0u8; entry.size];
        reader.read_exact(&mut data).ok()?;
        Some(data)
    }
}
