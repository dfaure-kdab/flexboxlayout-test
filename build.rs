fn main() {
    let config =
        slint_build::CompilerConfiguration::new().with_debug_info(true).with_style("fluent".into());
    slint_build::compile_with_config("ui/MainWindow.slint", config).expect("Slint build failed");
}
