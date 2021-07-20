use clap::{crate_name, crate_version, App, Arg};

pub fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about("A toy browser engine written in Rust")
        .arg(
            Arg::with_name("html-file")
                .help("HTML file to render.")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("File name to write output")
                .default_value("output.pdf"),
        )
}
