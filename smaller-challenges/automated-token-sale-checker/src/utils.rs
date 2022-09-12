use cargo_toml::{Manifest, Product};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Compiles a Scrypto package.
pub fn compile_package<P: AsRef<Path>>(package_dir: P) -> Result<Vec<u8>, PackageCompileError> {
    // build
    let status = Command::new("cargo")
        .current_dir(package_dir.as_ref())
        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
        .status()
        .map_err(|err| PackageCompileError::CompileError(err))?;
    if !status.success() {
        panic!("Failed to compile package: {:?}", package_dir.as_ref());
    }

    // resolve wasm name
    let mut cargo = package_dir.as_ref().to_owned();
    cargo.push("Cargo.toml");
    let manifest = Manifest::from_path(&cargo)
        .map_err(|err| PackageCompileError::CargoTomlManifestError(err))?;
    let wasm_name = if let Some(Product { name: Some(x), .. }) = manifest.lib {
        // lib name
        x
    } else if let Some(pkg) = manifest.package {
        // package name
        pkg.name.replace("-", "_")
    } else {
        // file name
        package_dir
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            .replace("-", "_")
    };

    // path of the wasm executable
    let mut path = PathBuf::from(package_dir.as_ref());
    path.push("target");
    path.push("wasm32-unknown-unknown");
    path.push("release");
    path.push(wasm_name);
    path.set_extension("wasm");

    // return
    Ok(fs::read(path).map_err(|err| PackageCompileError::FileReadError(err))?)
}

/// Represents an error encountered when compiling the package
#[derive(Debug)]
pub enum PackageCompileError {
    /// An error that occurs when the package could not be compiled.
    CompileError(std::io::Error),

    /// An error that occurs when dealing with the cargo_toml files.
    CargoTomlManifestError(cargo_toml::Error),

    /// An error that occurs when reading files
    FileReadError(std::io::Error),
}
