use super::MsgID;

use byteorder::{ByteOrder, LittleEndian};


/// USBTMC Main Header
/// 
pub struct Header {
    // Specifies the USBTMC message and the type of the USBTMC message. See Table 2. 
    msg_id: MsgID,
    b_tag: u8,
    b_tag_inverse: u8,
}

impl Header {
    
    /// Create a new USBTMC Header
    /// 
    pub fn new(msg_id: MsgID, b_tag: u8) -> Header {
        Header {
            msg_id,
            b_tag,
            b_tag_inverse: !b_tag & 0xFF,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![self.msg_id as u8, self.b_tag, self.b_tag_inverse, 0x00]
    }

    pub fn from_utf8(&self, bytes: &[u8]) -> Result<Header, String> {
        if bytes.len() < 4 {
            return Err("Header must be 4 bytes long".to_string());
        }

        let m_id = MsgID::from_u8(bytes[0])
            .ok_or(format!("Invalid MsgID '{}'", bytes[0]).as_str() )?;

        Ok(Header {
            msg_id: m_id.clone(),
            b_tag: bytes[1],
            b_tag_inverse: bytes[2],
        })
    }
}

// ----------------------------------------------------------------------------

/// USBTMC Device-Dependent Message Out Header
/// 
pub struct DevDepMsgOutHeader {
    // TransferSize
}

// ----------------------------------------------------------------------------

/// USBTMC Device-Dependent Message In Header
/// 
pub struct DevDepMsgInHeader {
    transferSize: u32,
    termChar: Option<u8>,
}

impl DevDepMsgInHeader {
    pub fn new(transfer_size: u32, term_char: Option<u8>) -> DevDepMsgInHeader {
        DevDepMsgInHeader {
            transferSize: transfer_size,
            termChar: term_char,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        //
        let mut hdr = vec![];
        hdr.append(&mut little_write_u32(self.transferSize, 4));

        // TermCharEnabled.
        // 1 - The Bulk-IN transfer must terminate
        // on the specified TermChar. The Host
        // may only set this bit if the USBTMC
        // interface indicates it supports
        // TermChar in the GET_CAPABILITIES
        // response packet.
        // 0 - The device must ignore TermChar. 
        if self.termChar.is_some() {
            hdr.push(0x2);
            hdr.push(self.termChar.unwrap());
        }
        else {
            hdr.push(0x0);
            hdr.push(0x0);
        }

        // Reserved
        hdr.append(&mut vec![0x00; 2]);

        hdr
    }
}

/// Write ti u32 in little endian
/// 
fn little_write_u32(size: u32, len: u8) -> Vec<u8> {
    let mut buf = vec![0; len as usize];
    LittleEndian::write_u32(&mut buf, size);
    buf
}


    