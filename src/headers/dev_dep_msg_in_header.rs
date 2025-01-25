use byteorder::{ByteOrder, LittleEndian};

use crate::logger::Logger;

/// USBTMC Device-Dependent Message In Header
/// 
pub struct DevDepMsgInHeader {
    transfer_size: u32,
    term_char: Option<u8>,
    eom: bool,
}

impl DevDepMsgInHeader {

    /// Create a new USBTMC Device-Dependent Message In Header
    /// 
    pub fn new(transfer_size: u32, term_char: Option<u8>) -> DevDepMsgInHeader {
        DevDepMsgInHeader {
            transfer_size,
            term_char,
            eom: false,
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
        //
        // Create trace logger
        let logger = Logger::new_for_crate();
        logger.trace(format!("DevDepMsgInHeader::from_u8_array({:?})", data));
        
        // if data.len()

        // Transfer size
        DevDepMsgInHeader {
            transfer_size : little_read_u32(&data[0..4]),
            term_char: None,
            eom: if data[4] & 0x1 == 0x1 { true } else { false },
        }
    }

    /// Getter for the transfer size
    /// 
    pub fn transfer_size(&self) -> u32 {
        self.transfer_size
    }

    /// Getter for the EOM signal
    /// 
    /// No other message after this one
    /// 
    pub fn is_eom(&self) -> bool {
        self.eom
    }

}

/// Write u32 in little endian
/// 
fn little_write_u32(size: u32, len: u8) -> Vec<u8> {
    let mut buf = vec![0; len as usize];
    LittleEndian::write_u32(&mut buf, size);
    buf
}

/// Read u32 in little endian
/// 
fn little_read_u32(data: &[u8]) -> u32 {
    LittleEndian::read_u32(data)
}
