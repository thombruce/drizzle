use std::fs;
use std::process::ExitCode;

/// Writes the CSS bundled at build time (see build.rs) into dist/.
fn main() -> ExitCode {
    if let Err(e) = write() {
        eprintln!("drizzle-build: {e}");
        return ExitCode::FAILURE;
    }
    println!(
        "✓ dist/drizzle.css ({} bytes)\n✓ dist/drizzle.min.css ({} bytes)",
        drizzle_css::CSS.len(),
        drizzle_css::CSS_MIN.len()
    );
    ExitCode::SUCCESS
}

fn write() -> std::io::Result<()> {
    fs::create_dir_all("dist")?;
    fs::write("dist/drizzle.css", drizzle_css::CSS)?;
    fs::write("dist/drizzle.min.css", drizzle_css::CSS_MIN)
}
