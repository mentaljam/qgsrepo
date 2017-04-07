#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        res.set_language(0x409);
        res.compile().unwrap();
    }
}

#[cfg(unix)]
fn main() { }
