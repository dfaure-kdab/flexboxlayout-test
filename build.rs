fn main() {
    // layout-order is still experimental upstream; the compiler reads this
    // env var when the configuration is created.
    unsafe { std::env::set_var("SLINT_ENABLE_EXPERIMENTAL_FEATURES", "1") };
    let config =
        slint_build::CompilerConfiguration::new().with_debug_info(true).with_style("fluent".into());
    slint_build::compile_with_config("ui/MainWindow.slint", config).expect("Slint build failed");
}
