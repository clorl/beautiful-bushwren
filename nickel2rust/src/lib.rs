#![allow(unused)]

use std::{
    path::PathBuf,
    fs::write
};

use ignore::{
    Walk, WalkBuilder, overrides::OverrideBuilder
};
use nickel_lang_core::{
    program::Program,
    files::Files,
    error::Sink,
    error::warning::Warning,
    eval::cache::lazy::CBNCache,
    eval::value::{
        ValueContent,
        lens::ValueLens
    },
    pretty::{
        Allocator,
        DocBuilder
    }
};

struct NickelError(nickel_lang_core::error::Error);
impl std::fmt::Display for NickelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Debug for NickelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<nickel_lang_core::error::Error> for NickelError {
    fn from(e: nickel_lang_core::error::Error) -> Self {
        NickelError(e)
    }
}

impl std::error::Error for NickelError {}

#[derive(Default)]
pub struct GenConfig {
    pub root: &'static str,
    pub patterns: Vec<&'static str>
}

impl GenConfig {
    fn walker(&self) -> Result<Walk, ignore::Error> {
        let mut ovbuilder = OverrideBuilder::new(&self.root);
        for f in &self.patterns {
            let _ = ovbuilder.add(f);
        }
        let overrides = ovbuilder.build()?;

        Ok(WalkBuilder::new(&self.root)
            .overrides(overrides)
            .build())
    }

    fn files(&self) -> Result<impl Iterator<Item = PathBuf>,ignore::Error> {
        self.walker().map(|walker| {
            walker.flatten()
                .filter(|de| de.file_type().is_some_and(|ft| ft.is_file()))
                .map(|fe| fe.into_path())
        })
    }
}

pub fn generate(config: GenConfig)-> Result<(), Box<dyn std::error::Error>>{
    let paths = config.files()?
    .filter_map(|p| p.canonicalize().ok());

    let reporter: Sink<(Warning, Files)> = Sink::default();
    let mut program: Program<CBNCache> = Program::new_from_files(paths, std::io::stderr(), reporter)?;
    let rt = program.eval_full_for_export().map_err(NickelError)?;
    beauty_dump("rt_check", &rt);
    // println!("----- Output -----");
    // println!("{rt:?}");
    // let output = format!("{rt:?}");
    // let pretty = output
    //     .replace(" { ", " {\n    ")
    //     .replace(" }, ", " },\n    ")
    //     .replace(", ", ",\n    ");
    // write("./debug_out.txt", pretty).ok();

    Ok(())
}

fn beauty_dump<T: std::fmt::Debug>(label: &str, val: T) {
    let raw = format!("{:#?}", val);
    let mut indent = 0;
    let mut out = String::new();

    for c in raw.chars() {
        match c {
            '{' | '[' | '(' => {
                indent += 1;
                out.push(c);
                out.push('\n');
                out.push_str(&"  ".repeat(indent));
            }
            '}' | ']' | ')' => {
                indent -= 1;
                out.push('\n');
                out.push_str(&"  ".repeat(indent));
                out.push(c);
            }
            ',' => {
                out.push(c);
                out.push('\n');
                out.push_str(&"  ".repeat(indent));
            }
            _ => out.push(c),
        }
    }
    // Clean up empty lines so it doesn't look like a mess
    out.push_str("//vim:ft=rust:fdm=indent:fdl=0");
    let cleaned = out.lines().filter(|l| !l.trim().is_empty()).collect::<Vec<_>>().join("\n");
    std::fs::write(format!("./debug_{label}.txt"), cleaned).ok();
}

#[cfg(test)]
mod tests {
    use super::*;
}
