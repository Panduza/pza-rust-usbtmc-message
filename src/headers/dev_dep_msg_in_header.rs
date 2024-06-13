use byteorder::{ByteOrder, LittleEndian};

/// USBTMC Device-Dependent Message In Header
/// 
pub struct DevDepMsgInHeader {
    transfer_size: u32,
    term_char: Option<u8>,
}

impl DevDepMsgInHeader {

    /// Create a new USBTMC Device-Dependent Message In Header
    /// 
    pub fn new(transfer_size: u32, term_char: Option<u8>) -> DevDepMsgInHeader {
        DevDepMsgInHeader {
            transfer_size,
            term_char,
        }
    }

    /// Convert to Vec<u8>, to be send over USB
    /// 
    pub fn to_vec(&self) -> Vec<u8> {
        // Create header
        let mut hdr = vec![];
        hdr.append(&mut little_write_u32(self.transfer_size, 4));

        // TermCharEnabled.
        // 1 - The Bulk-IN transfer must terminate
        // on the specified TermChar. The Host
        // may only set this bit if the USBTMC
        // interface indicates it supports
        // TermChar in the GET_CAPABILITIES
        // response packet.
        // 0 - The device must ignore TermChar. 
        if self.term_char.is_some() {
            hdr.push(0x2);
            hdr.push(self.term_char.unwrap());
        }
        else {
            hdr.push(0x0);
            hdr.push(0x0);
        }

        // Reserved
        hdr.append(&mut vec![0x00; 2]);
        hdr
    }


    pub fn from_u8_array(data: &[u8]) -> DevDepMsgInHeader {
        DevDepMsgInHeader {
            transfer_size : 0,
            term_char: None,
        }
    }

}

/// Write u32 in little endian
/// 
fn little_write_u32(size: u32, len: u8) -> Vec<u8> {
    let mut buf = vec![0; len as usize];
    LittleEndian::write_u32(&mut buf, size);
    buf
}


