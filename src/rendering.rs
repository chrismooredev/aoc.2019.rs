use std::ops::Index;



pub const CHAR_WIDTH: usize = 5;
pub const CHAR_HEIGHT: usize = 6;

// enable if adding letters
pub const DEBUG_OUTPUT: bool = true;

/// Rendered characters in form of unset bits being ascii 0's and set bits being ascii 1's
pub const RENDERED_CHARS: [[u8; CHAR_WIDTH*CHAR_HEIGHT]; 26] = [
	[b'0'; 5*6], // A
	[b'0'; 5*6], // B
    *concat_bytes!(
        b"01100",
        b"10010",
        b"10000",
        b"10000",
        b"10010",
        b"01100",
    ),
	[b'0'; 5*6], // D
    *concat_bytes!(
        b"11110",
        b"10000",
        b"11100",
        b"10000",
        b"10000",
        b"11110",
    ), // E
	*concat_bytes!(
		b"11110",
		b"10000",
		b"11100",
		b"10000",
		b"10000",
		b"10000",
	), // F
	*concat_bytes!(
		b"01100",
		b"10010",
		b"10000",
		b"10110",
		b"10010",
		b"01110",
	), // G
    *concat_bytes!(
        b"10010",
        b"10010",
        b"11110",
        b"10010",
        b"10010",
        b"10010",
    ), // H
	[b'0'; 5*6], // I
	*concat_bytes!(
		b"00110",
		b"00010",
		b"00010",
		b"00010",
		b"10010",
		b"01100",
	), // J
	[b'0'; 5*6], // K
	[b'0'; 5*6], // L
	[b'0'; 5*6], // M
	[b'0'; 5*6], // N
	[b'0'; 5*6], // O
    *concat_bytes!(
        b"11100",
        b"10010",
        b"10010",
        b"11100",
        b"10000",
        b"10000",
    ), // P
	[b'0'; 5*6], // Q
	[b'0'; 5*6], // R
	[b'0'; 5*6], // S
	[b'0'; 5*6], // T
	*concat_bytes!(
		b"10010",
		b"10010",
		b"10010",
		b"10010",
		b"10010",
		b"01100",
	), // U
	[b'0'; 5*6], // V
	[b'0'; 5*6], // W
	[b'0'; 5*6], // X
	[b'0'; 5*6], // Y
	*concat_bytes!(
		b"11110",
		b"00010",
		b"00100",
		b"01000",
		b"10000",
		b"11110",
	), // Z
];

/// accepts a byte buffer of raw input. Must be a multiple of CHAR_HEIGHT*CHAR_WIDTH.
/// Characters are read as from left to right, on one line. The buffer must use ascii
/// '0's for empty, and ascii '1's for set. No newline characters should be present.
pub fn parse<S: AsRef<[u8]>>(buffer: S) -> String {
    // assert_eq!(H, CHAR_HEIGHT);
    // assert_eq!(W % CHAR_WIDTH, 0);
    let buf = buffer.as_ref();
    assert_eq!(buf.len() % (CHAR_HEIGHT * CHAR_WIDTH), 0, "input buffer not a multiple of character size");
    let width = buf.len() / CHAR_HEIGHT;
    let letters = width / CHAR_WIDTH;
    let mut result = String::with_capacity(letters);
    for li in 0..letters {
        'letters: for gi in 0..RENDERED_CHARS.len() {
            let alpha = std::char::from_u32(b'A' as u32 + gi as u32).unwrap();
            let char = RENDERED_CHARS[gi];
            for row in 0..CHAR_HEIGHT {
                let row_start = row*width + li*CHAR_WIDTH;
                let rendered = &buf[row_start..row_start+CHAR_WIDTH];
                let reference = &char[row*CHAR_WIDTH..(row+1)*CHAR_WIDTH];

                if DEBUG_OUTPUT {
                    eprintln!("[LI={}][LETTER={},{}][ROW={}]", li, gi, alpha, row);
                    eprintln!("\t[RENDERED] = {:?}", rendered);
                    eprintln!("\t[REFERENC] = {:?}", reference);
                }
                if rendered != reference {
                    continue 'letters;
                }
            }
            
            // found good letter
            result.push(alpha);
            break;
        }
        if DEBUG_OUTPUT { eprintln!("after li={}, result={:?}", li, result); }

        if result.len() != li+1 {
            panic!("unable to determine letter {} from flattened input render (not yet recognized?)", li);
        }
    }

    result
}
