use std::fs;
use std::path::PathBuf;

fn main() {
    let problem_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src/problems")
        .to_string_lossy()
        .replace('\\', "/");
    let out_file = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("problem_mods.rs");

    let mut mods: Vec<String> = fs::read_dir(&problem_dir)
        .expect("failed to read src/problems")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with('p') && name.ends_with(".rs"))
        .map(|name| name.trim_end_matches(".rs").to_string())
        .collect();
    mods.sort();

    let content: String = mods
        .iter()
        .map(|m| {
            format!(
                "#[path = \"{problem_dir}/{m}.rs\"]\npub mod {m};\n"
            )
        })
        .collect();
    fs::write(out_file, content).expect("failed to write problem_mods.rs");

    println!("cargo:rerun-if-changed=src/problems");
}
