use std::fs::{self};

pub trait Renderer {
    fn render(&self, art: &str) -> Result<(), String>;
}

pub struct TerminalRenderer;

impl Renderer for TerminalRenderer {
    fn render(&self, art: &str) -> Result<(), String> {
        println!("{art}");
        Ok(())
    }
}

pub struct TextRenderer;

impl Renderer for TextRenderer {
    fn render(&self, art: &str) -> Result<(), String> {
        fs::write("ascii.txt", art).map_err(|err| format!("unable to write to file: {err}"))
    }
}
