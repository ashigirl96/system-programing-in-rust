use lib::io::SectionReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let reader = File::open("./Cargo.toml")?;
    let mut section_reader = SectionReader::new(reader, 14, 7)?;
    let mut writer = std::io::stdout();
    std::io::copy(&mut section_reader, &mut writer);
    Ok(())
}
