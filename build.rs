use anyhow::*;
use fs_extra::{copy_items, dir::CopyOptions};
use std::env;

fn main() -> Result<()> {
    // Tells cargo to rebuild if items in res folder are changed
    println!("cargo:rerun-if-changed=test_files/*");

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("test_files/");
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}