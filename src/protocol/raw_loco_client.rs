/*
 * Created on Mon Nov 30 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

//todo error handle

use std::io::{Read, Write};

use super::RawLocoPacket;
use std::ops::{Deref, DerefMut};

/// Like BufReader and BufWriter, provide optimized Command read/write operation to stream.
pub struct RawLocoClient<S: Read + Write> {
    pub stream: S,
    read_buffer: [u8; 2048],
}

impl<S: Read + Write> RawLocoClient<S> {
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            read_buffer: [0u8; 2048],
        }
    }

    pub fn read_raw_loco_packet(&mut self) -> Option<RawLocoPacket> {
        match self.stream.read(&mut self.read_buffer) {
            Ok(read_bytes) => {
                if read_bytes < 22 {
                    return None;
                }
                let buffer = &self.read_buffer[0..read_bytes];
                Some(bincode::deserialize(buffer).unwrap())
            }
            Err(_) => None,
        }
    }

    pub fn write_raw_loco_packet(&mut self, raw_loco_packet: RawLocoPacket) {
        let mut cursor = Vec::with_capacity(raw_loco_packet.header.data_size as usize + 22);
        bincode::serialize_into(&mut cursor, &raw_loco_packet).unwrap();

        self.write_all(&cursor).unwrap();
    }
}

impl<T: Write + Read> Deref for RawLocoClient<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl<T: Write + Read> DerefMut for RawLocoClient<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}
