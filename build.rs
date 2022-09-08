// build.rs

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("test.ico")
        .set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}