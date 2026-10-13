use anyhow::Result;
use std::fs::File;
use std::io::Write;
use zip::ZipArchive;

fn main() -> Result<()> {
    // Usage: cargo run --example check_rebuilt_zip -- path/to/input.zip [path/to/output.zip]
    let mut args = std::env::args().skip(1);
    let input = args
        .next()
        .expect("Usage: check_rebuilt_zip <input.zip> [output.zip]");
    let output = args.next().unwrap_or_else(|| "rebuilt.zip".into());

    println!("Opening input: {}", input);
    let in_file = File::open(&input)?;
    let (mut rebuilt_file, excluded) = crate::util::wiiload::rebuild_zip(in_file)
        .context("rebuild_zip failed")?;

    // Write rebuilt tempfile to a named file so we can inspect it
    let mut out = File::create(&output)?;
    std::io::copy(&mut rebuilt_file, &mut out)?;
    println!("Wrote rebuilt zip to: {}", output);

    // Re-open the written file and list its contents
    let out_for_read = File::open(&output)?;
    let mut archive = ZipArchive::new(out_for_read)?;
    println!("Files in rebuilt zip ({} entries):", archive.len());
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        println!(" - {}", file.name());
    }

    println!("\nFiles excluded from the app bundle (kept out of rebuilt zip):");
    for e in excluded {
        println!(" - {}", e);
    }

    Ok(())
}
