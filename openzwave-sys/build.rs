extern crate cc;

fn main() {
    let libopenzwave = pkg_config::probe_library("libopenzwave").unwrap();

    let _c = cc::Build::new()
        .file("openzwave-c/options.cc")
        .file("openzwave-c/manager.cc")
        .file("openzwave-c/notification.cc")
        .file("openzwave-c/value_classes/value_id.cc")
        .cpp(true)
        .flag("-std=c++17") // to iterate with ranges
        .flag_if_supported("-Wno-unused-private-field")
        .includes(libopenzwave.include_paths)
        .compile("libopenzwave-c.a");
}
