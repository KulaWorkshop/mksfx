use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::spu;

pub struct SoundEntry {
    pub buffer: Vec<u8>,
    pub pitch_value: u32,
    pub frequency: u32,
}

pub struct SFXFile {
    pub entries: Vec<SoundEntry>,
    pub semitone_table: Vec<u16>,
    pub fine_table: Vec<u16>,
}

impl SFXFile {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            semitone_table: Self::load_table("00809C87AC8F379845A1DCAA04B5C8BF2FCB44D711E4A1F1"),
            fine_table: Self::load_table(
                "00800E801D802C803B804A8058806780768085809480A380B180C080CF80DE80ED80FC800B811A8129813881468155816481738182819181A081AF81BE81CD81DC81EB81FA810982188227823682458254826382728282829182A082AF82BE82CD82DC82EB82FA820A83198328833783468355836483748383839283A183B083C083CF83DE83ED83FD830C841B842A843A84498458846884778486849584A584B484C384D384E284F1840185108520852F853E854E855D856D857C858B859B85AA85BA85C985D985E885F8850786178626863686458655866486748683869386A286B286C186D186E086F08600870F871F872E873E874E875D876D877D878C87",
            ),
        }
    }

    fn load_table(hex: &str) -> Vec<u16> {
        hex.as_bytes()
            .chunks(4)
            .filter_map(|chunk| {
                if chunk.len() == 4 {
                    let lo =
                        u8::from_str_radix(std::str::from_utf8(&chunk[0..2]).ok()?, 16).ok()?;
                    let hi =
                        u8::from_str_radix(std::str::from_utf8(&chunk[2..4]).ok()?, 16).ok()?;
                    Some(u16::from_le_bytes([lo, hi]))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // open file
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // parse entries
        let sound_count = reader.read_u32::<LittleEndian>()?;
        let mut entries = Vec::new();
        for _ in 0..sound_count {
            let offset = reader.read_u32::<LittleEndian>()?;
            let pitch_value = reader.read_u32::<LittleEndian>()?;
            entries.push((offset, pitch_value));
        }

        let mut sfx_file = SFXFile::new();
        for (i, (offset, pitch_value)) in entries.iter().enumerate() {
            // determine size of sound data buffer
            let end = entries.get(i + 1).map(|(off, _)| *off);
            let size = end.map(|e| e - offset).unwrap_or_else(|| {
                (reader.seek(SeekFrom::End(0)).unwrap() - *offset as u64) as u32
            });

            // reader buffer
            reader.seek(SeekFrom::Start(*offset as u64))?;
            let mut buffer = vec![0u8; size as usize];
            reader.read_exact(&mut buffer)?;

            // calculate frequency
            let frequency =
                spu::pitch_value2hz(*pitch_value, &sfx_file.semitone_table, &sfx_file.fine_table);

            // add entry
            sfx_file.entries.push(SoundEntry {
                buffer,
                pitch_value: *pitch_value,
                frequency: frequency.floor() as u32,
            });
        }

        Ok(sfx_file)
    }

    pub fn pack(&self, path: &Path) -> Result<(), Error> {
        // open file
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // write sound count
        let sound_count = self.entries.len() as u32;
        writer.write_u32::<LittleEndian>(sound_count)?;

        // write entries without offsets
        for entry in &self.entries {
            writer.write_u32::<LittleEndian>(0)?;
            writer.write_u32::<LittleEndian>(entry.pitch_value)?;
        }

        // write sound data and offsets
        for (i, entry) in self.entries.iter().enumerate() {
            let offset = writer.stream_position()? as u32;
            writer.write_all(&entry.buffer)?;

            writer.seek(SeekFrom::Start((4 + (i * 8)) as u64))?;
            writer.write_u32::<LittleEndian>(offset)?;
            writer.seek(SeekFrom::End(0))?;
        }

        Ok(())
    }
}
