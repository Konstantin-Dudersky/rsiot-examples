fn main() {
    // fluent-dark
    // material-dark
    // cupertino-dark
    // cosmic-dark

    let config = slint_build::CompilerConfiguration::new().with_style("cupertino-dark".into());
    slint_build::compile_with_config("main.slint", config).unwrap();
}
