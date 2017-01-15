use std::io::{ self, Cursor, Write };

use byteorder::{ BigEndian, ReadBytesExt, WriteBytesExt };
use tokio_core::io::{ Codec, EasyBuf };

pub struct LpmCodec;

impl Codec for LpmCodec {
    type In = EasyBuf;
    type Out = Vec<u8>;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let len = Cursor::new(&buf).read_u32::<BigEndian>();
        match len {
            Ok(len) => {
                let len = len as usize;
                if len + 4 < buf.len() {
                    buf.drain_to(4); // discard the length
                    Ok(Some(buf.drain_to(len)))
                } else {
                    Ok(None)
                }
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::UnexpectedEof {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.write_u32::<BigEndian>(msg.len() as u32)?;
        buf.write_all(&msg)?;
        Ok(())
    }
}
