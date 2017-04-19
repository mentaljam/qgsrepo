# qgsrepo
A simple QGIS repository generator written in Rust.

[![License](https://img.shields.io/badge/license-zlib-blue.svg)]()
[![License](https://img.shields.io/github/release/mentaljam/qgsrepo.svg)]()

A single binary that allows to generate a QGIS repository index XML file for a plugins directory by extracting metadata from plugins archives.

## Usage
    qgsrepo [OPTIONS] ROOT URL
#### Positional arguments:
    root                  a directory containing plugin archives
    url                   a repository url for the "download_url" entry
#### Optional arguments:
    -h,--help             show this help message and exit
    -o,--output OUTPUT    an output file name, default is "plugins.xml" in a
                          repository root
    --skip_icons          do not extract icons
    --icons_dir ICONS_DIR a root subdirectory for icons, default is "icons"
    -s,--strict           strict metadata check
    -f,--force            rewrite an output file if exists
    -v,--version          show version and exit
    
## Building
1. [Install](https://www.rust-lang.org/en-US/install.html) the Rust compiler and Cargo:
2. Run the build command `cargo build --release`
