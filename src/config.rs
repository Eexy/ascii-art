use std::{path::PathBuf, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum PaletteSize {
    Long,
    Short,
}

impl FromStr for PaletteSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "long" => Ok(PaletteSize::Long),
            "short" => Ok(PaletteSize::Short),
            _ => Err(format!("unknown palette {s}, expected: long, short")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RendererOutput {
    Terminal,
    Text,
}

impl FromStr for RendererOutput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "terminal" => Ok(RendererOutput::Terminal),
            "text" => Ok(RendererOutput::Text),
            _ => Err(format!("unknown renderer {s}, expected: terminal, text")),
        }
    }
}

const VALID_FILE_EXTENSION: [&str; 3] = ["jpeg", "jpg", "png"];

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub image_path: PathBuf,
    pub palette_size: PaletteSize,
    pub renderer: RendererOutput,
}

impl Config {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        args.next();

        let mut image_path: Option<PathBuf> = None;
        let mut palette_size = PaletteSize::Long;
        let mut renderer = RendererOutput::Terminal;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-p" | "--palette" => {
                    palette_size = args
                        .next()
                        .ok_or_else(|| {
                            "parameter `-p` needs a value: 'short' or 'long'".to_string()
                        })?
                        .parse()?;
                }
                "-r" | "--renderer" => {
                    renderer = args
                        .next()
                        .ok_or_else(|| {
                            "parameter `-r` needs a value: 'terminal' or 'text'".to_string()
                        })?
                        .parse()?;
                }
                flag if flag.starts_with('-') => return Err(format!("unknown parameter {flag}")),
                _ if image_path.is_some() => return Err("a path was already provided".to_string()),
                _ => image_path = Some(Config::parse_path(&arg)?),
            }
        }

        Ok(Config {
            image_path: image_path.ok_or_else(|| "no image path was provided".to_string())?,
            palette_size,
            renderer,
        })
    }

    fn parse_path(arg: &str) -> Result<PathBuf, String> {
        let path = PathBuf::from(arg);
        let is_valid_file_extension =
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    VALID_FILE_EXTENSION
                        .iter()
                        .any(|valid_extension| ext.eq_ignore_ascii_case(valid_extension))
                });

        if !is_valid_file_extension {
            return Err(format!(
                "invalid file extension, file must be {}",
                VALID_FILE_EXTENSION.join(", "),
            ));
        }

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_when_no_path() {
        let args = [String::from("program_name")].into_iter();
        assert_eq!(
            Config::parse(args).err(),
            Some(String::from("no image path was provided"))
        );
    }

    #[test]
    fn error_when_duplicate_path() {
        let args = [
            String::from("program_name"),
            String::from("image_1.png"),
            String::from("image_2.png"),
        ]
        .into_iter();
        assert_eq!(
            Config::parse(args).err(),
            Some(String::from("a path was already provided"))
        );
    }

    #[test]
    fn error_when_unknown_flag() {
        let args = [String::from("program_name"), String::from("-x")].into_iter();

        assert_eq!(
            Config::parse(args).err(),
            Some(String::from("unknown parameter -x"))
        );
    }

    #[test]
    fn error_when_missing_value_for_parameter() {
        let args = [String::from("program_name"), String::from("-p")].into_iter();

        assert_eq!(
            Config::parse(args).err(),
            Some(String::from(
                "parameter `-p` needs a value: 'short' or 'long'"
            ))
        );
    }

    #[test]
    fn error_when_invalid_file_extension() {
        let args = [String::from("program_name"), String::from("file.gif")].into_iter();

        assert_eq!(
            Config::parse(args).err(),
            Some(format!(
                "invalid file extension, file must be {}",
                VALID_FILE_EXTENSION.join(", ")
            ))
        );
    }

    #[test]
    fn error_when_invalid_value_for_parameter() {
        let args = [
            String::from("program_name"),
            String::from("-p"),
            String::from("huge"),
        ]
        .into_iter();

        assert_eq!(
            Config::parse(args).err(),
            Some(String::from("unknown palette huge, expected: long, short"))
        );
    }

    #[test]
    fn parse_valid_config() {
        let args = [
            String::from("program_name"),
            String::from("test.png"),
            String::from("-p"),
            String::from("short"),
            String::from("-r"),
            String::from("text"),
        ]
        .into_iter();

        let expected_config = Config {
            image_path: PathBuf::from("test.png"),
            palette_size: PaletteSize::Short,
            renderer: RendererOutput::Text,
        };

        assert_eq!(Config::parse(args), Ok(expected_config));
    }

    #[test]
    fn parse_valid_config_in_any_order_and_longer_form() {
        let args = [
            String::from("program_name"),
            String::from("-p"),
            String::from("short"),
            String::from("test.png"),
            String::from("--renderer"),
            String::from("text"),
        ]
        .into_iter();

        let expected_config = Config {
            image_path: PathBuf::from("test.png"),
            palette_size: PaletteSize::Short,
            renderer: RendererOutput::Text,
        };

        assert_eq!(Config::parse(args), Ok(expected_config));
    }

    #[test]
    fn parse_valid_config_on_uppercase_name() {
        let args = [
            String::from("program_name"),
            String::from("-p"),
            String::from("short"),
            String::from("TEST.PNG"),
            String::from("--renderer"),
            String::from("text"),
        ]
        .into_iter();

        let expected_config = Config {
            image_path: PathBuf::from("TEST.PNG"),
            palette_size: PaletteSize::Short,
            renderer: RendererOutput::Text,
        };

        assert_eq!(Config::parse(args), Ok(expected_config));
    }
}
