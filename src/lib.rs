use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
pub struct Args {
    pub source_path: PathBuf,
    pub target_apth: PathBuf,
}

pub fn read_file(file_path: &Path) -> String {
    std::fs::read_to_string(file_path).expect("could not read file")
}
fn extract_lines_from_file(file_path: &Path) -> Vec<String> {
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::fixture::FileWriteStr;

    #[test]
    fn split_into_lines_returns_correct_count() -> Result<(), Box<dyn std::error::Error>> {
        let source = assert_fs::NamedTempFile::new("source.txt")?;
        source.write_str("line1\nline2\nline3")?;

        let list_of_lines = extract_lines_from_file(source.path());

        assert_eq!(list_of_lines.len(), 3);
        Ok(())
    }

    #[test]
    fn extract_lines_returns_one_element_for_empty_file() -> Result<(), Box<dyn std::error::Error>>
    {
        let source = assert_fs::NamedTempFile::new("source.txt")?;
        source.write_str("")?;

        let list_of_lines = extract_lines_from_file(source.path());

        assert_eq!(list_of_lines.len(), 3);
        Ok(())
    }
}
