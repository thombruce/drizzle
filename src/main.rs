use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use lightningcss::bundler::{Bundler, FileProvider};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions};
use lightningcss::targets::{Browsers, Targets};

fn main() -> ExitCode {
    let project_root: PathBuf = std::env::current_dir().expect("cwd");
    let entry = project_root.join("css").join("drizzle.css");
    let out_dir = project_root.join("dist");

    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!("failed to create dist/: {e}");
        return ExitCode::FAILURE;
    }

    let targets = Targets::from(Browsers {
        chrome: Some(95 << 16),
        firefox: Some(90 << 16),
        safari: Some(15 << 16),
        edge: Some(95 << 16),
        ..Browsers::default()
    });

    let unmin = match bundle_and_print(&entry, targets.clone(), false) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::FAILURE;
        }
    };
    let min = match bundle_and_print(&entry, targets, true) {
        Ok(s) => s,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::FAILURE;
        }
    };

    let unmin_path = out_dir.join("drizzle.css");
    let min_path = out_dir.join("drizzle.min.css");

    if let Err(e) = fs::write(&unmin_path, &unmin) {
        eprintln!("write {}: {e}", unmin_path.display());
        return ExitCode::FAILURE;
    }
    if let Err(e) = fs::write(&min_path, &min) {
        eprintln!("write {}: {e}", min_path.display());
        return ExitCode::FAILURE;
    }

    println!(
        "✓ {} ({} bytes)\n✓ {} ({} bytes)",
        unmin_path.display(),
        unmin.len(),
        min_path.display(),
        min.len()
    );

    ExitCode::SUCCESS
}

fn bundle_and_print(entry: &Path, targets: Targets, minify: bool) -> Result<String, String> {
    let fs_provider = FileProvider::new();
    let parser_options = ParserOptions::default();
    let mut bundler = Bundler::new(&fs_provider, None, parser_options);
    let mut stylesheet = bundler
        .bundle(entry)
        .map_err(|e| format!("bundle error: {e}"))?;

    if minify {
        stylesheet
            .minify(MinifyOptions {
                targets: targets.clone(),
                ..MinifyOptions::default()
            })
            .map_err(|e| format!("minify error: {e}"))?;
    }

    let printer_opts = PrinterOptions {
        minify,
        targets: targets.clone(),
        ..PrinterOptions::default()
    };
    let result = stylesheet
        .to_css(printer_opts)
        .map_err(|e| format!("print error: {e}"))?;
    Ok(result.code)
}
