extern crate argparse;

use self::argparse::{
    ArgumentParser,
    Store,
    StoreTrue,
    StoreFalse,
    Print
};


#[derive(Debug)]
pub struct Config {
    pub force:     bool,
    pub strict:    bool,
    pub withicons: bool,
    pub reporoot:  String,
    pub repourl:   String,
    pub outname:   String,
    pub iconsdir:  String
}

impl Config {
    pub fn new() -> Config {
        Config {
            force:     false,
            strict:    false,
            withicons: true,
            reporoot:  String::new(),
            repourl:   String::new(),
            outname:   String::from("plugins.xml"),
            iconsdir:  String::from("icons")
        }
    }

    pub fn parse_args(&mut self) {
        let mut ap = ArgumentParser::new();
        ap.set_description("Generates the QGIS repository index file.");
        ap.refer(&mut self.reporoot)
            .add_argument(&"root", Store,
                          "a directory containing plugin archives")
            .required();
        ap.refer(&mut self.repourl)
            .add_argument(&"url", Store,
                          "a repository url for the \"download_url\" entry")
            .required();
        ap.refer(&mut self.outname)
            .add_option(&["-o", "--output"], Store,
                        "an output file name, default is \"plugins.xml\" in a repository root");
        ap.refer(&mut self.withicons)
            .add_option(&["--no-icons"], StoreFalse,
                        "do not extract icons");
        ap.refer(&mut self.iconsdir)
            .add_option(&["--icons-dir"], Store,
                        "a root subdirectory for icons, default is \"icons\"");
        ap.refer(&mut self.strict)
            .add_option(&["-s", "--strict"], StoreTrue,
                        "strict metadata check");
        ap.refer(&mut self.force)
            .add_option(&["-f", "--force"], StoreTrue,
                        "rewrite an output file if exists");
        ap.add_option(&["-v", "--version"],
                      Print(env!("CARGO_PKG_VERSION").to_string()), "show version and exit");
        ap.parse_args_or_exit();
    }
}
