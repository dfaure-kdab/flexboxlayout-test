// Include the slint-generated code
slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    TestCase::new().unwrap().run()?;
    Ok(())
}
