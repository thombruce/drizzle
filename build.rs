use std::fs;
use std::path::{Path, PathBuf};

use lightningcss::bundler::{Bundler, FileProvider};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions};
use lightningcss::targets::{Browsers, Targets};

fn main() {
    println!("cargo:rerun-if-changed=css/");

    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let entry = manifest_dir.join("css").join("drizzle.css");

    let targets = Targets::from(Browsers {
        chrome: Some(95 << 16),
        firefox: Some(90 << 16),
        safari: Some(15 << 16),
        edge: Some(95 << 16),
        ..Browsers::default()
    });

    let unmin = bundle(&entry, targets.clone(), false).expect("drizzle-css: bundle failed");
    let min = bundle(&entry, targets, true).expect("drizzle-css: minify failed");

    fs::write(out_dir.join("drizzle.css"), unmin).expect("drizzle-css: write drizzle.css failed");
    fs::write(out_dir.join("drizzle.min.css"), min)
        .expect("drizzle-css: write drizzle.min.css failed");
}

fn bundle(entry: &Path, targets: Targets, minify: bool) -> Result<String, String> {
    let fs_provider = FileProvider::new();
    let mut bundler = Bundler::new(&fs_provider, None, ParserOptions::default());
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

    stylesheet
        .to_css(PrinterOptions {
            minify,
            targets,
            ..PrinterOptions::default()
        })
        .map(|r| r.code)
        .map_err(|e| format!("print error: {e}"))
}
