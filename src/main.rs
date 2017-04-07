extern crate zip;
extern crate ini;
extern crate xml;

mod config;
mod qgsmeta;

use std::path::PathBuf;
use std::fs;
use zip::ZipArchive;
use ini::Ini;
use std::io::Read;
use xml::writer;
use qgsmeta::{
    MetaEntries,
    metakey,
    xmlkey
};


#[derive(Debug)]
enum ExitCodes {
    Success = 0,
    NoRootDir,
    NoOutDir,
    FileExists,
    NoIconsDir
}

macro_rules! exit_with_code {
    ($code:path) => (std::process::exit($code as i32))
}


macro_rules! write_url {
    ($writer:ident, $cfg:ident) => {
        let url_tag = writer::XmlEvent::start_element(qgsmeta::xmlkey(&MetaEntries::DownloadUrl));
        let url = writer::XmlEvent::characters($cfg.repourl.as_str());
        $writer.write(url_tag).unwrap();
        $writer.write(url).unwrap();
        $writer.write(writer::XmlEvent::end_element()).unwrap();
    }
}

macro_rules! write_file {
    ($writer:ident, $zip_name:ident) => {
        let file_tag = writer::XmlEvent::start_element(qgsmeta::xmlkey(&MetaEntries::FileName));
        let zip_name = writer::XmlEvent::characters($zip_name.to_str().unwrap());
        $writer.write(file_tag).unwrap();
        $writer.write(zip_name).unwrap();
        $writer.write(writer::XmlEvent::end_element()).unwrap();
    }
}

macro_rules! write_icon {
    ($writer:ident, $cfg:ident, $icon_tag_name:ident, $icon_name:ident) => {
        let icon_tag = writer::XmlEvent::start_element($icon_tag_name);
        let icon_path = PathBuf::from(&$cfg.iconsdir).join(&$icon_name);
        let icon_text = writer::XmlEvent::characters(icon_path.to_str().unwrap());
        $writer.write(icon_tag).unwrap();
        $writer.write(icon_text).unwrap();
        $writer.write(writer::XmlEvent::end_element()).unwrap();
    }
}

macro_rules! write_entries {
    ($writer:ident, $section:ident, $entries:ident, $func:path) => {
        for entry in &$entries {
            match $section.get(metakey(&entry)) {
                Some(value) => if !value.is_empty() {
                    let tag = writer::XmlEvent::start_element(qgsmeta::xmlkey(&entry));
                    $writer.write(tag).unwrap();
                    $writer.write($func(value)).unwrap();
                    $writer.write(writer::XmlEvent::end_element()).unwrap();
                },
                None => ()
            }
        }
    }
}


fn main() {

    let mut cfg = config::Config::new();
    cfg.parse_args();

    let root = PathBuf::from(&cfg.reporoot);
    if !root.is_dir() {
        println!("Error: the root directory does not exist: \"{:?}\"", root);
        exit_with_code!(ExitCodes::NoRootDir);
    }

    let outpath = {
        if cfg.outname == "plugins.xml" {
            root.join(&cfg.outname)
        }
        else {
           let file = PathBuf::from(&cfg.outname);
           {
               let dir = file.parent().unwrap();
               if !dir.is_dir() {
                   println!("Error: the output file directory does not exist: \"{:?}\"", dir);
                   exit_with_code!(ExitCodes::NoOutDir);
                }
           }
           file
        }
    };
    if !cfg.force && outpath.is_file() {
        println!("Error: the output file already exists. Run with the -f option to overwrite: {:?}", outpath);
        exit_with_code!(ExitCodes::FileExists);
    }

    let iconsdir = root.join(&cfg.iconsdir);
    if cfg.withicons && !iconsdir.is_dir() {
        println!("Error: the icon directory does not exist: {:?}", iconsdir);
        exit_with_code!(ExitCodes::NoIconsDir);
    }

    let mut outfile = fs::File::create(outpath).unwrap();
    let mut xmlwriter = writer::EmitterConfig::new().perform_indent(true).create_writer(&mut outfile);
    {
        let plugins = writer::XmlEvent::start_element("plugins");
        xmlwriter.write(plugins).unwrap();
    }

    let attr_entries  = attr_entries!();
    let text_entries  = text_entries!();
    let cdata_entries = cdata_entries!();

    let entries = fs::read_dir(root).unwrap();
    let mut icons = Vec::new();
    for entry in entries {
        let path = entry.unwrap().path();
        if !path.is_file() || path.extension().unwrap() != "zip" {
            continue
        }
        let zipfile = fs::File::open(&path).unwrap();
        let zipname = path.file_name().unwrap();
        println!("Processing: {:?}", zipname);
        let mut zipreader = ZipArchive::new(&zipfile).unwrap();

        let plugin_dir = match zipreader.by_index(0) {
            Result::Ok(zipentry) => {
                let entry_path = PathBuf::from(zipentry.name());
                let mut path_comps = entry_path.iter();
                path_comps.next().unwrap().to_string_lossy().into_owned()
            },
            Result::Err(err) => {
                println!("Warning: could not read zip, skipping: {}", err);
                continue
            },
        };

        let metadata_text = {
            let metadata_path = format!("{}/metadata.txt", plugin_dir);
            match zipreader.by_name(metadata_path.as_str()) {
                Result::Ok(mut metadata) => {
                    let mut md = String::new();
                    metadata.read_to_string(&mut md).unwrap();
                    // This is a hook to process files with multiline values (changelog)
                    md.push_str("\ndummy=dummy");
                    md
                },
                Result::Err(err) => {
                    println!("Warning: could not read the \"metadata.txt\", skipping: {}", err);
                    continue
                }
            }
        };

        let metadata = match Ini::load_from_str(metadata_text.as_str()) {
            Result::Ok(metadata) => {
                metadata
            },
            Result::Err(err) => {
                println!("Warning: could not parse plugin metadata, skipping: {:?}", err);
                continue
            }
        };

        let general = match metadata.section(Some("general".to_owned())) {
            Some(section) => section,
            None => {
                println!("Warning: metadata file does not contain the \"general\" section, skipping");
                continue
            }
        };

        if cfg.strict {
            let mut ok = true;
            for entry in required_entries!() {
                let key = metakey(&entry);
                if !general.contains_key(key) {
                    println!("Warning: strict check - metadata file does not contain the \"{}\" entry", key);
                    ok = false;
                    break
                }
            }
            if !ok {
                println!("Warning: strict check - skipping plugin due to bad metadata");
                continue
            }
        }

        {
            let mut pyqgis_plugin = writer::XmlEvent::start_element("pyqgis_plugin");
            let mut ok = true;
            for attr in &attr_entries {
                let key = metakey(&attr);
                match general.get(key) {
                    Some(value) => pyqgis_plugin = pyqgis_plugin.attr(xmlkey(&attr), value),
                    None => {
                        println!("Warning: metadata file does not contain the required \"{}\" entry", key);
                        ok = false;
                    }
                }
            }
            if ok {
                xmlwriter.write(pyqgis_plugin).unwrap();
            } else {
                println!("Warning: skipping plugin due to bad metadata");
                continue
            }
        }

        write_url!(xmlwriter, cfg);
        write_file!(xmlwriter, zipname);
        write_entries!(xmlwriter, general, text_entries, writer::XmlEvent::characters);
        write_entries!(xmlwriter, general, cdata_entries, writer::XmlEvent::cdata);

        if cfg.withicons {
            let icon_tag_name = metakey(&MetaEntries::Icon);
            let zipicon = match general.get(icon_tag_name) {
                Some(zipicon) => zipicon,
                None => continue
            };
            let zipicon_path = PathBuf::from(format!("{}/{}", plugin_dir, zipicon));
            let ext = zipicon_path.extension().unwrap();
            let icon_name = {
                let mut icon_name = PathBuf::from(&zipname);
                icon_name.set_extension(&ext);
                icon_name.to_owned()
            };
            icons.push(icon_name.as_os_str().to_owned());
            write_icon!(xmlwriter, cfg, icon_tag_name, icon_name);
            let icon_path = iconsdir.join(&icon_name);
            if icon_path.exists() {
                continue
            }
            match zipreader.by_name(zipicon_path.to_str().unwrap()) {
                Result::Ok(icon) => {
                    let mut icon_reader = icon;
                    let mut icon_writer = fs::File::create(&icon_path).unwrap();
                    match std::io::copy(&mut icon_reader, &mut icon_writer) {
                        Result::Err(err) =>
                            println!("Warning: could not extract plugin icon: {:?} - {}", zipicon_path, err),
                        _ => ()
                    }
                },
                Result::Err(err) =>
                    println!("Warning: could not read plugin icon: {:?} - {}", zipicon_path, err)
            }
        }

        xmlwriter.write(writer::XmlEvent::end_element()).unwrap();
    }

    xmlwriter.write(writer::XmlEvent::end_element()).unwrap();

    if cfg.withicons {
        println!("Removing obsolete icons");
        let allicons = fs::read_dir(iconsdir).unwrap();
        for entry in allicons {
            let entry_reader = entry.unwrap();
            let entryname = &entry_reader.file_name();
            let mut remove = true;
            for icon in &icons {
                if entryname == icon {
                    remove = false;
                    break
                }
            }
            if remove {
                match fs::remove_file(entry_reader.path()) {
                    Result::Ok(_) => println!("{:?}", entryname),
                    Result::Err(err) => println!("Warning: could not remove obsolete icon: {}", err),
                }
            }
        }
    }

    exit_with_code!(ExitCodes::Success);
}
