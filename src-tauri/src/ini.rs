//! bf2_visor.ini: guarda la ubicación del último .ske y .skinnedmesh usados.
use std::fs;
use std::io;
use std::path::Path;

/// Devuelve (ske, skinnedmesh). Vacío si no existe o no está definido.
pub fn read(path: &Path) -> (String, String) {
    let text = match fs::read_to_string(path) { Ok(t) => t, Err(_) => return (String::new(), String::new()) };
    let (mut ske, mut arms) = (String::new(), String::new());
    for line in text.lines() {
        let l = line.trim().trim_start_matches('\u{feff}');
        if l.is_empty() || l.starts_with(';') || l.starts_with('#') || l.starts_with('[') { continue; }
        if let Some((k, v)) = l.split_once('=') {
            let v = v.trim().trim_matches('"').to_string();
            match k.trim().to_lowercase().as_str() {
                "ske" => ske = v,
                "skinnedmesh" | "arms" => arms = v,
                _ => {}
            }
        }
    }
    (ske, arms)
}

pub fn write(path: &Path, ske: &str, arms: &str) -> io::Result<()> {
    let body = format!(
        "; BF2 Visor - ultimos archivos usados.\r\n; Se actualiza solo; tambien podes editarlo a mano.\r\n[ultimo]\r\nske={}\r\nskinnedmesh={}\r\n",
        ske, arms
    );
    fs::write(path, body)
}
