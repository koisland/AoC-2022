use std::{error::Error, fs};

use itertools::Itertools;

const PACKET_LEN: usize = 4;
const MSG_LEN: usize = 14;
type Packet = (char, char, char, char);

#[derive(Debug)]
struct BufferReadError {}

impl std::fmt::Display for BufferReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't read buffer.")
    }
}

impl Error for BufferReadError {}

pub fn read_comm_packet(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let mut first_marker: Option<usize> = None;

    for (i, (char_1, char_2, char_3, char_4)) in
        contents.chars().tuple_windows::<Packet>().enumerate()
    {
        let uniq_chars = [&char_1, &char_2, &char_3, &char_4]
            .into_iter()
            .unique()
            .collect_vec();
        if uniq_chars.len() == PACKET_LEN {
            // println!("After {} chars - {char_1}{char_2}{char_3}{char_4}", i + PACKET_LEN);
            first_marker = Some(i + PACKET_LEN);
            break;
        }
    }
    if let Some(marker_idx) = first_marker {
        Ok(marker_idx)
    } else {
        Err(Box::new(BufferReadError {}))
    }
}

pub fn read_comm_message(fname: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(fname)?;
    let mut first_marker: Option<usize> = None;

    for i in 0..contents.len() {
        if let Some(message_marker) = contents.get(i..i + MSG_LEN) {
            let message_chars = message_marker.chars().unique().collect_vec();
            if message_chars.len() == MSG_LEN {
                println!("After {} chars - {message_marker}", i + MSG_LEN);
                first_marker = Some(i + MSG_LEN);
                break;
            }
        }
    }
    if let Some(marker_idx) = first_marker {
        Ok(marker_idx)
    } else {
        Err(Box::new(BufferReadError {}))
    }
}
