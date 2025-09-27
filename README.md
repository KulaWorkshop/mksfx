<h4 align="center">
    <a href="https://github.com/KulaWorkshop/mksfx/releases">Releases</a> |
    <a href="https://docs.kula.quest/tools/mksfx">Documentation</a>
</h4>

<div align="center">

# mksfx

[![MIT License](https://img.shields.io/badge/License-MIT-db8c2c.svg)](https://opensource.org/licenses/MIT)
![Version](https://img.shields.io/badge/Version-1.0.0-db8c2c)

**MKSFX** is a command-line tool that can extract and build **.SFX** files from **Kula Quest**.

</div>

## Features

-   Supports every version of the game - Kula World, Roll Away, Kula Quest, ...
-   Supports both the extraction and creation of **.SFX** files.
-   Automatically decodes and encodes **SPU-ADPCM** audio into and from **.WAV** files for simple extraction and creation.
-   Uses a **configuration file** for simplistic sound management and pitch value adjustment.

## Installation

The latest pre-compiled binaries are available for download on the [releases page](https://github.com/KulaWorkshop/mksfx/releases).

## Usage

To extract an SFX file, use the **extract** command followed by its path and an output directory:

```bash
mksfx extract HIRO.SFX output
```

This will extract the sound files from the SFX file into a directory, and will also save a **yaml configuration** file in the directory:

```yaml
sounds:
- filename: sound001.wav
  pitch_value: 6400
- filename: sound002.wav
  pitch_value: 3328
- filename: sound003.wav
  pitch_value: 3328
- filename: sound004.wav
  pitch_value: 6400
- filename: sound005.wav
  pitch_value: 8704
  ...
```

You can make any adjustments to the config file or audio files as you'd like, just **be sure** that you encode your custom WAV files properly and that you update the pitch value if needed (See the [FFmpeg](#ffmpeg) section for more information).

To create an SFX file, use the **build** command followed by the path to create it and the path to a config file:

```bash
mksfx build CUSTOM.SFX output/build.yaml
```

> [!IMPORTANT]
> Make sure that your sound files are in the same directory as the config file.

If you would like to extract the sounds as raw **SPU-ADPCM** files, you can use the `--format raw` flag:

```bash
mksfx extract HIRO.SFX output --format raw
```

When importing a raw sound, be sure to set `format: raw` in the config file for that sound:

```yaml
- filename: sound010.raw
  format: raw
  pitch_value: 7936
```

## FFmpeg

To import a custom sound file, it must be prepared in the correct format.
[FFmpeg](https://en.wikipedia.org/wiki/FFmpeg) is a popular tool used for converting audio files, and can be downloaded [here](https://www.ffmpeg.org/download.html) for your platform.
Only the `ffmpeg` executable is needed for this process.

The following command will encode an audio file, in this case a `coin.mp3` file as an example, into the required format for mksfx — a **mono** (1 audio channel) **signed 16-bit PCM WAV** file:

```bash
ffmpeg -i coin.mp3 -ac 1 -ar 22050 -sample_fmt s16 output_coin.wav
```

This command will also set the sample rate to **22050hz**, so when importing this sound be sure to set the pitch value to **6040**.
Here is a table with some common sample rates and their associated pitch value:

| Sample Rate | Pitch Value |
| ----------- | ----------- |
| 11681hz     | 3328        |
| 22050hz     | 6040        |
| 23363hz     | 6400        |
| 33042hz     | 7936        |
| 39287hz     | 8704        |
| 44100hz     | 9216        |

In most cases, just using the example FFmpeg command above with a pitch value of 6040 will be sufficient for custom sounds.

## Credits

Developed by **[SaturnKai](https://saturnkai.dev/)**. Additionally, this project uses **libpsxav** to perform the encoding process, which is licensed under the [zlib](https://github.com/WonderfulToolchain/psxavenc/blob/main/LICENSE) license.

-   Copyright (c) 2019 Ben "GreaseMonkey" Russell
-   Copyright (c) 2019, 2020, 2023 Adrian "asie" Siekierka

## Changelog

**Version 1.0.0 (2025-09-26)**

-   Initial release.
