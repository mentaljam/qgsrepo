#[cfg(windows)]
extern crate winres;
#[cfg(windows)]
extern crate strfmt;

#[cfg(windows)]
use std::env::var;
#[cfg(windows)]
use std::path::PathBuf;
#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::io::{
    Read,
    Write
};
#[cfg(windows)]
use strfmt::strfmt;
#[cfg(windows)]
use std::collections::HashMap;

#[cfg(all(windows, target_pointer_width = "32"))]
macro_rules! arch {
    () => ("x86");
    (win) => ("win32");
}

#[cfg(all(windows, target_pointer_width = "64"))]
macro_rules! arch {
    () => ("x64");
    (win) => ("win64");
}


#[cfg(windows)]
fn main() {
    if var("PROFILE").unwrap() == "release" {
        // compile .rc file
        let mut res = winres::WindowsResource::new();
        res.set_language(0x409);
        res.compile().unwrap();

        // prepare for building msi package
        let cargo_manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
        // wix source dir
        let wix_src_dir = PathBuf::from(&cargo_manifest_dir).join("wix");
        // wix build dir
        let wix_build_path = PathBuf::from(&var("OUT_DIR").unwrap()).join("wix");
        // create wix build dir if does't exist
        if !wix_build_path.is_dir() {
            fs::create_dir(&wix_build_path).unwrap();
        }

        // copy wix files
        let mut wxs_files = Vec::new();
        let mut cultures = Vec::new();
        {
            let files = fs::read_dir(&wix_src_dir).unwrap();
            for file in files {
                let path = file.unwrap().path();
                if !path.is_file() {
                    continue
                }
                let ext = path.extension().unwrap().to_str().unwrap();
                let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
                let copy = match ext {
                    "wxs" => {
                        wxs_files.push(stem);
                        true
                    },
                    "wxl" => {
                        cultures.push(stem);
                        true
                    },
                    "wxi" => true,
                    "rtf" => true,
                    _ => false
                };
                if copy {
                    let name = path.file_name().unwrap();
                    fs::copy(&path, wix_build_path.join(name)).unwrap();
                }
            }
        }

        // write build_msi.bat
        {
            let version = var("CARGO_PKG_VERSION").unwrap();
            let build_bat_in_path = wix_src_dir.join("build_msi.bat.in");
            let mut build_bat_in_reader = fs::File::open(build_bat_in_path).unwrap();
            let mut build_bat_data = String::new();
            build_bat_in_reader.read_to_string(&mut build_bat_data).unwrap();
            let build_dir = PathBuf::from(&cargo_manifest_dir).join("target\\release");
            let build_bat_path = build_dir.join("build_msi.bat");
            let mut vars = HashMap::new();
            vars.insert("arch".to_string(),         arch!().to_string());
            vars.insert("win_arch".to_string(),     arch!(win).to_string());
            vars.insert("out_dir".to_string(),      wix_build_path.to_str().unwrap().to_string());
            vars.insert("target_dir".to_string(),   build_dir.to_str().unwrap().to_string());
            vars.insert("version".to_string(),      version);
            vars.insert("wxs_files".to_string(),    wxs_files.join(", "));
            vars.insert("cultures".to_string(),     cultures.join(", "));
            build_bat_data = strfmt(&build_bat_data, &vars).unwrap();
            let mut build_bat_writer = fs::File::create(build_bat_path).unwrap();
            build_bat_writer.write_all(&mut build_bat_data.as_bytes()).unwrap();
        }
    }
}

#[cfg(unix)]
fn main() { }
