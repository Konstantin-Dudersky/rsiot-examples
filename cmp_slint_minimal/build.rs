fn main() {
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("src/config_slint/app/global/main.slint", config).unwrap()
}
