use std::path::Path;

use miniz_oxide::deflate::compress_to_vec_zlib;
use walkdir::WalkDir;

pub fn encode(input_dir: &str, output_file: &str, profile: Option<&str>) -> anyhow::Result<()> {
    let profile = profile.unwrap_or_else(|| {
        Path::new(output_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("profile_0")
    });

    let profile = Path::new(profile);

    let files = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".json"))
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    result.extend_from_slice(&[b'D', b'D', b'V', b'1']);
    result.extend_from_slice(&(files.len() as u32).to_le_bytes());

    for file in files {
        result.push(b'A');
        let name = file.path().strip_prefix(input_dir)?;
        let name = profile.join(name);
        let name_bytes = name.to_string_lossy();
        let name_bytes = name_bytes.as_bytes();
        if name_bytes.len() > 255 {
            anyhow::bail!("File name is too long: {name:?}");
        }

        let remainder = 256 - name_bytes.len();
        result.extend_from_slice(name_bytes);
        result.resize(result.len() + remainder, 0);

        let json = std::fs::read(file.path())?;
        let json = compress_to_vec_zlib(&json, 4);

        result.push(b'B');
        result.extend_from_slice(&(json.len() as u32).to_le_bytes());

        result.push(b'C');
        result.extend(json);

        result.push(b'D');

        println!("Encoding file {name:?}");
    }

    result.push(b'Z');
    result.resize(1024 * 1024, 0);

    std::fs::write(output_file, &result)?;

    Ok(())
}
