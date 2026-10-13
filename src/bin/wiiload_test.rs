use std::fs::File;
use std::io::{Write, Seek, SeekFrom};
use zip::write::ZipWriter;
use zip::CompressionMethod;
use zip::write::FileOptions;
use anyhow::Result;

fn create_input_zip(path: &str) -> Result<()> {
    let mut f = File::create(path)?;
    let mut z = ZipWriter::new(&mut f);

    let opts = FileOptions::default()
        .compression_method(CompressionMethod::Stored);

    // top-level directory name + two files
    z.start_file("LibreShop/boot.dol", opts)?;
    z.write_all(b"BOOT-DOL-DATA")?;

    z.start_file("LibreShop/meta.xml", opts)?;
    z.write_all(b"<app><name>LibreShop</name></app>")?;

    z.finish()?;
    // ensure file is flushed
    f.seek(SeekFrom::Start(0))?;
    Ok(())
}

fn main() -> Result<()> {
    // 1) create input.zip in workspace
    create_input_zip("input.zip")?;

    // 2) open and call rebuild_zip from your code
    let input = File::open("input.zip")?;
    let (mut out_file, excluded) = crate::util::wiiload::rebuild_zip(input)?;

    // 3) write returned File to output.zip for inspection
    let mut out = File::create("output.zip")?;
    std::io::copy(&mut out_file, &mut out)?;
    println!("Excluded files from rebuild_zip: {:#?}", excluded);

    Ok(())
}
