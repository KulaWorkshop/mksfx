pub fn pitch_value2hz(stored_value: u32, semitone_table: &[u16], fine_table: &[u16]) -> f64 {
    let root_note = 0x24;
    let center_fine = 0x00;
    let note = (stored_value / 256) as i32;
    let note_fine = (stored_value & 0xFF) as i32;

    let pitch_val = spu_note2pitch(
        root_note,
        center_fine,
        note,
        note_fine,
        semitone_table,
        fine_table,
    );

    (pitch_val as f64 / 4096.0) * 44100.0
}

pub fn spu_note2pitch(
    param1: i32,
    param2: i32,
    param3: i32,
    param4: i32,
    semitone_table: &[u16],
    fine_table: &[u16],
) -> u16 {
    let ivar5 = param3 + ((param4 + param2) >> 7) - param1;
    let svar2 = if ivar5 < 0 && ivar5 % 12 != 0 {
        ivar5 / 12 - 1
    } else {
        ivar5 / 12
    };
    let mut svar6 = svar2 - 2;

    let mut ivar5 = ((ivar5 % 12) + 12) % 12;

    if ivar5 < 0 {
        ivar5 += 12;
        svar6 -= 1;
    }

    if svar6 >= 0 {
        semitone_table[ivar5 as usize]
    } else {
        let uvar1 = (-svar6) as u32;
        let sem = semitone_table[ivar5 as usize] as u32;
        let fine = fine_table[((param4 + param2) & 0x7F) as usize] as u32;

        let mut val = (sem * fine) >> 16;

        val += 1 << ((uvar1 - 1) & 0x1F);

        val >>= uvar1 & 0x1F;
        (val & 0xFFFF) as u16
    }
}
