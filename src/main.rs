use std::{
    env::{self, Args},
    path::PathBuf,
};

const VALID_FILE_EXTENSION: [&str; 3] = ["jpeg", "jpg", "png"];

fn get_file_path(mut args: Args) -> Result<PathBuf, String> {
    args.next();
    let raw_path = args
        .next()
        .ok_or_else(|| "You need to provide a path to the image".to_string())?;

    let path = PathBuf::from(raw_path);
    let is_valid_file_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| VALID_FILE_EXTENSION.contains(&ext.to_lowercase().as_str()));

    if !is_valid_file_extension {
        return Err(format!(
            "Invalid file extension. file must be {}",
            VALID_FILE_EXTENSION.join(", "),
        ));
    }

    Ok(path)
}

fn main() -> Result<(), String> {
    let path = get_file_path(env::args())?;
    println!("{}", path.display());

    Ok(())
}
