use std::io::{ self, Write };

use byteorder::{ LittleEndian, WriteBytesExt};

pub struct Submessage(pub SubmessageType, pub Vec<u8>);

bitflags! {
  flags SubmessageType : u8 {
    // const PAD = 0x01, /* Pad */
    const LITTLEENDIAN = 0x01, /* Xavier's Endianness hack? */
    const ACKNACK = 0x06, /* AckNack */
    const HEARTBEAT = 0x07, /* Heartbeat */
    const GAP = 0x08, /* Gap */
    const INFO_TS = 0x09, /* InfoTimestamp */
    const INFO_SRC = 0x0c, /* InfoSource */
    const INFO_REPLY_IP4 = 0x0d, /* InfoReplyIp4 */
    const INFO_DST = 0x0e, /* InfoDestination */
    const INFO_REPLY = 0x0f, /* InfoReply */
    const NACK_FRAG = 0x12, /* NackFrag */
    const HEARTBEAT_FRAG = 0x13, /* HeartbeatFrag */
    const DATA = 0x15, /* Data */
    const DATA_FRAG = 0x16, /* DataFrag */
  }
}

impl Submessage {
  pub fn serialize<W: Write>(&self, mut w: &mut W) -> io::Result<()> {
    let mut flags = self.0;
    // flags.toggle(LITTLEENDIAN);
    flags = flags | LITTLEENDIAN;

    let flags_arr = [flags.bits];
    try!(w.write_all(&flags_arr[..]));

    try!(w.write_u16::<LittleEndian>(self.1.len() as u16));

    w.write_all(&self.1[..])
  }
}