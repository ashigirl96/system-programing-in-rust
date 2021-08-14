use anyhow::Result;

fn main() -> Result<()> {
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    writer.write_record(&["Hello", "World"]);
    writer.write_record(&["Japan", "Tokyo", "Minato"]);
    writer.flush();
    Ok(())
}
