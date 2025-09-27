mod adpcm;
mod args;
mod config;
mod libpsxav;
mod sfx;
mod spu;

use crate::sfx::SoundEntry;
use args::Commands;
use clap::Parser;
use colored::Colorize;
use config::SoundFile;
use sfx::SFXFile;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    // set virtual terminal
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    // print header
    println!(
        "{} v{} by Brandon Gardenhire\n",
        env!("CARGO_PKG_NAME").bold(),
        env!("CARGO_PKG_VERSION")
    );

    // parse arguments
    let arguments = args::Arguments::parse();

    // handle command
    let start = Instant::now();
    if let Err(e) = match arguments.command {
        Commands::Extract {
            input_sfx,
            output_dir,
            format,
        } => handle_extract(input_sfx, output_dir, format),
        Commands::Build {
            input_config,
            output_sfx,
        } => handle_build(input_config, output_sfx),
    } {
        eprintln!("\n{} {}.", "error:".bold().red(), e);
        std::process::exit(1);
    }

    println!("{} ({:?})", "Done.".green().bold(), start.elapsed());
}

fn handle_build(input: String, output: String) -> Result<(), Box<dyn Error>> {
    // read config
    let config_path = Path::new(&input);
    let config_dir = config_path.parent().unwrap_or(Path::new("."));
    let config =
        config::read_config(config_path).map_err(|e| format!("invalid config file - \"{}\"", e))?;

    // add entries
    let mut sfx_file = SFXFile::new();
    for entry in config.sounds {
        println!("{}", &entry.filename);

        let entry_path = config_dir.join(&entry.filename);
        let buffer = if entry.format == args::SoundFormat::Wav {
            adpcm::encode_from_wav(&entry_path)
                .map_err(|e| format!("invalid wav file - \"{}\"", e))?
        } else {
            fs::read(entry_path)?
        };

        sfx_file.entries.push(SoundEntry {
            buffer,
            pitch_value: entry.pitch_value,
            frequency: 0,
        });
    }

    // write sfx file
    sfx_file.pack(Path::new(&output))?;

    println!(
        "\nCreated file with {} sounds.",
        sfx_file.entries.len().to_string().bold()
    );

    Ok(())
}

fn handle_extract(
    input: String,
    output: Option<String>,
    format: args::SoundFormat,
) -> Result<(), Box<dyn Error>> {
    // open sfx file
    let mut sound_files = Vec::<SoundFile>::new();
    let sfx_file = SFXFile::from_file(input)?;

    // create output directory
    let output_dir = output.unwrap_or(String::from("."));
    let output_dir = Path::new(&output_dir);
    fs::create_dir_all(output_dir)?;

    for (i, entry) in sfx_file.entries.iter().enumerate() {
        let sound_filename = format!("sound{:03}.{}", i + 1, format);
        let output_path = output_dir.join(&sound_filename);

        // write audio file
        println!("{} ({}hz)", &sound_filename, entry.frequency);
        if format == args::SoundFormat::Wav {
            adpcm::decode_to_wav(output_path, &entry.buffer, entry.frequency)?;
        } else {
            fs::write(output_path, &entry.buffer)?;
        }

        sound_files.push(SoundFile {
            filename: sound_filename.clone(),
            pitch_value: entry.pitch_value,
            format,
        });
    }

    // write config
    config::write_config(&output_dir.join("build.yaml"), sound_files)?;

    println!(
        "\nExtracted {} sounds.",
        sfx_file.entries.len().to_string().bold()
    );

    Ok(())
}
