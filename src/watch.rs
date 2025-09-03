use crate::config::SiteConfig;
use crate::error::Result;
use crate::generate::generate;
use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

/// Sets up a RAM-based directory for storing generated files during development.
///
/// This function creates a temporary directory in RAM (/dev/shm) to store the generated
/// site files instead of writing them to the SSD. This helps prevent SSD wear during
/// development when files are frequently regenerated.
fn setup_ramdisk(config: &SiteConfig) -> Result<()> {
    // Create a directory in /dev/shm which is already a tmpfs
    let out_dir = "/dev/shm/lepkefing_out";
    if Path::new(out_dir).exists() {
        fs::remove_dir_all(out_dir)?;
    }
    fs::create_dir_all(out_dir)?;

    // Create a symlink from ./out to our ramdisk
    let current_dir = std::env::current_dir()?;
    let out_path = current_dir.join(&config.output_dir);
    if out_path.exists() {
        if out_path.is_symlink() {
            fs::remove_file(&out_path)?;
        } else {
            fs::remove_dir_all(&out_path)?;
        }
    }
    std::os::unix::fs::symlink(out_dir, out_path)?;

    Ok(())
}

fn setup_regular_output(config: &SiteConfig) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let out_path = current_dir.join(&config.output_dir);
    if !out_path.exists() {
        fs::create_dir_all(&out_path)?;
    }
    Ok(())
}

fn get_latest_modification_time(dir: &Path) -> Result<SystemTime> {
    if !dir.exists() {
        return Ok(SystemTime::UNIX_EPOCH);
    }

    let mut latest = SystemTime::UNIX_EPOCH;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let modified = metadata.modified()?;

        if modified > latest {
            latest = modified;
        }

        if metadata.is_dir() {
            let sub_latest = get_latest_modification_time(&entry.path())?;
            if sub_latest > latest {
                latest = sub_latest;
            }
        }
    }

    Ok(latest)
}

pub fn watch(site_name: &str, use_ramdisk: bool, config: &SiteConfig) -> Result<()> {
    let site_path = format!("{}/{site_name}", config.sites_base_dir);
    let site_dir = Path::new(&site_path);

    if !site_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Site directory '{site_path}' does not exist"),
        )
        .into());
    }

    println!("\nOutput directory setup:");
    if use_ramdisk {
        #[cfg(target_os = "linux")]
        {
            println!("✓ RAM disk requested: yes");
            println!("✓ RAM disk available: yes (Linux detected)");
            println!("Setting up RAM-based output directory...");
            setup_ramdisk(config)?;
            println!("✓ RAM disk enabled: using /dev/shm/lepkefing_out");
        }
        #[cfg(not(target_os = "linux"))]
        {
            println!("✓ RAM disk requested: yes");
            println!("✗ RAM disk available: no (Linux required)");
            println!("Falling back to regular disk output...");
            setup_regular_output(config)?;
            println!("✓ Using regular disk output: ./out");
        }
    } else {
        println!("✓ RAM disk requested: no");
        println!("✓ Using regular disk output: ./out");
        setup_regular_output(config)?;
    }

    // Generate the site once initially
    println!("\nGenerating initial site...");
    if let Err(e) = generate(site_name, config) {
        eprintln!("Error generating initial site: {e}");
    }

    println!("\nWatching for changes...");
    println!("Press Ctrl+C to stop");
    println!("Monitoring site: {site_path}");

    let mut last_modified = get_latest_modification_time(site_dir)?;

    loop {
        let current_modified = get_latest_modification_time(site_dir)?;
        if current_modified > last_modified {
            println!("\nChanges detected, regenerating site...");
            if let Err(e) = generate(site_name, config) {
                eprintln!("Error generating site: {e}");
            } else {
                println!("Site regenerated successfully!");
            }
            last_modified = current_modified;
        }
        thread::sleep(Duration::from_secs(1));
    }
}
