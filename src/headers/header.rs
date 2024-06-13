use crate::MsgID;

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

    /// Convert from u8 array
    /// 
    pub fn from_u8_array(data: &[u8]) -> Result<Header, String> {
        // Check if data is long enough
        if data.len() < 4 {
            return Err( format!("Header must be 4 bytes long (provided={})", data.len()) );
        }

        let m_id = MsgID::from_u8_array(data[0])
            .ok_or(format!("Invalid MsgID '{}'", data[0]).as_str() )?;

        Ok(Header {
            msg_id: m_id.clone(),
            b_tag: data[1],
            b_tag_inverse: data[2],
        })
    }
}

// #[test]
// fn test_add() {
//     assert_eq!(add(1, 2), 3);
// }


