# QgsRepo
A simple QGIS repository generator written in Rust.

[![License](https://img.shields.io/badge/license-zlib-blue.svg)](https://github.com/mentaljam/qgsrepo/blob/master/LICENSE.txt)
[![Release](https://img.shields.io/github/release/mentaljam/qgsrepo.svg)](https://github.com/mentaljam/qgsrepo/releases/latest)
[![Snap Status](https://build.snapcraft.io/badge/mentaljam/qgsrepo.svg)](https://build.snapcraft.io/user/mentaljam/qgsrepo)

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
    --no-icons            do not extract icons
    --icons-dir ICONS_DIR a root subdirectory for icons, default is "icons"
    -s,--strict           strict metadata check
    -f,--force            rewrite an output file if exists
    -v,--version          show version and exit
    
## Building
1. [Install](https://www.rust-lang.org/en-US/install.html) the Rust compiler and Cargo
2. Clone sources: `git clone git@github.com:mentaljam/qgsrepo.git`
3. Enter the source directory: `cd qgsrepo`
4. Run cargo build command: `cargo build --release`

## Packaging

### Windows installer
After build run `target\release\build_msi.bat` ([WiX toolset](http://wixtoolset.org/) is required).

The latest windows installer package is available in the [releases](https://github.com/mentaljam/qgsrepo/releases/latest) section.

### Linux
On Linux systems QgsRepo can be installed with a snap package from The Ubuntu Store: `sudo snap install qgsrepo`.
