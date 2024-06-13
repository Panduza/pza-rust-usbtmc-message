use super::MsgID;

/// USBTMC Main Header
/// 
pub struct Header {
    // Specifies the USBTMC message and the type of the USBTMC message. See Table 2. 
    msg_id: MsgID,
    b_tag: u8,
    b_tag_inverse: u8,

}

impl Header {
    fn new(msg_id: MsgID, b_tag: u8) -> Header {
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


struct DevDepMsgOutHeader {

}

