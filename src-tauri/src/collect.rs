//! Port de bf2_bridge.py: busca los archivos relacionados con un .con/.mesh y los devuelve
//! como pares (ruta_relativa_en_minúsculas, ruta_absoluta).
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Found {
    pub target: String,
    pub ske: String,
    pub arms: String,
    pub files: Vec<(String, String)>,
}

const ALLOWED: [&str; 9] = ["con", "tweak", "bundledmesh", "staticmesh", "skinnedmesh", "ske", "inc", "baf", "dds"];

fn clean(p: &Path) -> PathBuf {
    let c = fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let s = c.to_string_lossy().to_string();
    if let Some(rest) = s.strip_prefix(r"\\?\") { PathBuf::from(rest) } else { c }
}

fn lname(p: &Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default()
}

pub fn find_mod_root(target: &Path) -> PathBuf {
    let cur = target.parent().unwrap_or(Path::new("."));
    for p in cur.ancestors() {
        let name = lname(p);
        if name == "objects" || name == "objects_client" || name == "objects_server" {
            return match p.parent() { Some(pp) if pp.exists() => pp.to_path_buf(), _ => p.to_path_buf() };
        }
        if p.join("Objects").exists() || p.join("objects").exists() { return p.to_path_buf(); }
    }
    cur.parent().unwrap_or(cur).to_path_buf()
}

fn walk(dir: &Path, depth: usize, max_depth: Option<usize>, mod_root: &Path, out: &mut BTreeMap<String, String>) {
    if let Some(m) = max_depth { if depth > m { return; } }
    let rd = match fs::read_dir(dir) { Ok(r) => r, Err(_) => return };
    for e in rd.flatten() {
        let ft = match e.file_type() { Ok(t) => t, Err(_) => continue };
        let path = e.path();
        if ft.is_dir() { walk(&path, depth + 1, max_depth, mod_root, out); continue; }
        let ext = path.extension().map(|x| x.to_string_lossy().to_lowercase()).unwrap_or_default();
        if !ALLOWED.contains(&ext.as_str()) { continue; }
        let rel = match path.strip_prefix(mod_root) {
            Ok(r) => r.to_string_lossy().replace('\\', "/"),
            Err(_) => path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        };
        out.insert(rel.to_lowercase(), path.to_string_lossy().to_string());
    }
}

pub fn collect(target_file: &Path, ske_cfg: &str, arms_cfg: &str) -> Found {
    let target = clean(target_file);
    let mod_root = find_mod_root(&target);
    let mut asset_dir = target.parent().unwrap_or(Path::new(".")).to_path_buf();
    if ["meshes", "animations", "ai", "textures"].contains(&lname(&asset_dir).as_str()) {
        if let Some(p) = asset_dir.parent() { asset_dir = p.to_path_buf(); }
    }
    let mut files: BTreeMap<String, String> = BTreeMap::new();
    walk(&asset_dir, 0, None, &mod_root, &mut files);
    if mod_root != asset_dir {
        let mut roots: Vec<PathBuf> = vec![];
        for sub in ["Soldiers", "soldiers", "Animations", "animations", "Common", "common", "Weapons", "weapons", "Vehicles", "vehicles", "Objects", "objects"] {
            for base in [mod_root.join("Objects").join(sub), mod_root.join(sub)] {
                if base.exists() && base != asset_dir && !roots.contains(&base) { roots.push(base); }
            }
        }
        for r in roots { walk(&r, 0, Some(5), &mod_root, &mut files); }
    }

    let exists = |s: &str| !s.is_empty() && Path::new(s).exists();
    let mut ske = if exists(ske_cfg) { ske_cfg.to_string() } else { String::new() };
    if ske.is_empty() {
        ske = files.iter().find(|(k, _)| k.ends_with("1p_setup.ske")).or_else(|| files.iter().find(|(k, _)| k.ends_with(".ske"))).map(|(_, v)| v.clone()).unwrap_or_default();
    }
    let mut arms = if exists(arms_cfg) { arms_cfg.to_string() } else { String::new() };
    if arms.is_empty() {
        arms = files.iter().find(|(k, _)| k.contains("soldier") && k.ends_with(".skinnedmesh")).or_else(|| files.iter().find(|(k, _)| k.ends_with(".skinnedmesh"))).map(|(_, v)| v.clone()).unwrap_or_default();
    }
    let mut ske_key = String::new();
    let mut arms_key = String::new();
    if exists(&ske) { ske_key = format!("saved/{}", lname(Path::new(&ske))); files.insert(ske_key.clone(), ske.clone()); }
    if exists(&arms) { arms_key = format!("saved/{}", lname(Path::new(&arms))); files.insert(arms_key.clone(), arms.clone()); }

    let target_rel = match target.strip_prefix(&mod_root) {
        Ok(r) => r.to_string_lossy().replace('\\', "/").to_lowercase(),
        Err(_) => lname(&target),
    };
    files.insert(target_rel.clone(), target.to_string_lossy().to_string());

    Found { target: target_rel, ske: ske_key, arms: arms_key, files: files.into_iter().collect() }
}
