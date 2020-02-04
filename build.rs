fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        embed_resource::compile("windres/calc.rc");
    }
}
