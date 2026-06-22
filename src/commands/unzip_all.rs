use anyhow::Result;
use clap::Parser;
use std::io;
use std::path::{Component, Path, PathBuf};

#[derive(Parser)]
pub struct Args {
    /// Directory to search for .zip files
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,

    /// Delete .zip files after extraction
    #[arg(short, long)]
    pub delete: bool,

    /// Overwrite existing extraction directories
    #[arg(short, long)]
    pub force: bool,
}

pub fn run(args: Args) -> Result<()> {
    let mut found = false;

    for entry in std::fs::read_dir(&args.dir)? {
        let entry = entry?;
        let zip_path = entry.path();

        if zip_path.extension().and_then(|s| s.to_str()) != Some("zip") {
            continue;
        }

        found = true;

        let zip_stem = zip_path.file_stem().unwrap().to_string_lossy().to_string();

        let extract_dir = args.dir.join(&zip_stem);

        if extract_dir.exists() {
            if args.force {
                std::fs::remove_dir_all(&extract_dir)?;
            } else {
                eprintln!(
                    "Skip: '{}' already exists (use --force to overwrite)",
                    extract_dir.display()
                );
                continue;
            }
        }

        std::fs::create_dir_all(&extract_dir)?;
        println!("Unzip: {} -> {}/", zip_path.display(), zip_stem);

        unzip(&zip_path, &extract_dir)?;

        if args.delete {
            std::fs::remove_file(&zip_path)?;
            println!("Deleted: {}", zip_path.display());
        }
    }

    if !found {
        println!("No .zip files found.");
    }

    Ok(())
}

fn unzip(zip_path: &Path, extract_dir: &Path) -> Result<()> {
    let zip_file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let raw_name = entry.name().to_owned();

        // Guard against path traversal (e.g. ../../evil)
        let Some(safe_path) = sanitize_path(&raw_name) else {
            continue;
        };
        let out_path = extract_dir.join(&safe_path);

        if raw_name.ends_with('/') {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out_file = std::fs::File::create(&out_path)?;
            io::copy(&mut entry, &mut out_file)?;
        }
    }

    Ok(())
}

// Strip leading `/`, `.`, and any `..` components to prevent path traversal.
fn sanitize_path(raw: &str) -> Option<PathBuf> {
    let mut safe = PathBuf::new();

    for component in Path::new(raw).components() {
        match component {
            Component::Normal(part) => safe.push(part),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {}
            Component::CurDir => {}
        }
    }

    if safe.as_os_str().is_empty() {
        return None;
    }

    Some(safe)
}
