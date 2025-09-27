use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "mksfx", about)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Extract an SFX file to a specified directory
    Extract {
        /// Path to an SFX file
        input_sfx: String,
        /// Output directory for extracted sound files (default: current directory)
        output_dir: Option<String>,
        /// Audio output format (default: wav) (options: wav, raw)
        #[arg(short, long, default_value_t = SoundFormat::Wav, hide_possible_values = true, hide_default_value = true)]
        format: SoundFormat,
    },
    /// Build an SFX file from a config file
    Build {
        /// Path to a YAML config file
        input_config: String,
        /// Path for the output SFX file
        output_sfx: String,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoundFormat {
    Wav,
    Raw,
}

impl std::fmt::Display for SoundFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundFormat::Wav => write!(f, "wav"),
            SoundFormat::Raw => write!(f, "raw"),
        }
    }
}
