use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,

    #[arg(short, long)]
    pub delete: bool,

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
        let zip_name = zip_path.file_stem().unwrap().to_string_lossy().to_string();

        let extract_dir = args.dir.join(&zip_name);
        std::fs::create_dir_all(&extract_dir)?;

        println!("Unzip: {}", zip_path.display());

        let zip_file = std::fs::File::open(&zip_path)?;
        let mut archive = zip::ZipArchive::new(zip_file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let filename = file.name();

            let mut file_path = PathBuf::from(filename);

            // file_path = sa
            //
            // if file_path.components().any(|c| c.as_os_str() == "..") {
            //     file_path = sa
            // }

            if filename.ends_with('/') {
                std::fs::create_dir_all(&file_path)?;
            } else {
                if let Some(p) = file_path.parent() {
                    std::fs::create_dir_all(p)?;
                }
                let mut outfile = std::fs::File::create(&file_path)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        if args.delete {
            std::fs::remove_file(&zip_path)?;
        }
    }

    if !found {
        println!("Any .zip not found.");
    }

    Ok(())
}
