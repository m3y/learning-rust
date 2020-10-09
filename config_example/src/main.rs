fn main() {
    println!(
        "{:#?}",
        std::env::var_os("EXAMPLE_CONFIG_DIR")
            .map(std::path::PathBuf::from)
            .or_else(|| dirs::config_dir().map(|d| d.join("example")))
    )
}
