use slint_build::CompilerConfiguration;

fn main() {
    slint_build::compile_with_config(
        "ui/main.slint",
        CompilerConfiguration::new().with_style("cosmic".to_string()),
    )
    .unwrap();
}
