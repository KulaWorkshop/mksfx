use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::args;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub sounds: Vec<SoundFile>,
}

#[derive(Serialize, Deserialize)]
pub struct SoundFile {
    pub filename: String,
    #[serde(default = "default_format", skip_serializing_if = "is_default_format")]
    pub format: args::SoundFormat,
    pub pitch_value: u32,
}

pub fn write_config(path: &Path, sound_files: Vec<SoundFile>) -> Result<(), Box<dyn Error>> {
    let config = Config {
        sounds: sound_files,
    };

    let yaml = serde_yaml::to_string(&config)?;
    fs::write(path, yaml)?;

    Ok(())
}

pub fn read_config(path: &Path) -> Result<Config, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&config)?;
    Ok(config)
}

fn default_format() -> args::SoundFormat {
    args::SoundFormat::Wav
}

fn is_default_format(f: &args::SoundFormat) -> bool {
    *f == args::SoundFormat::Wav
}
