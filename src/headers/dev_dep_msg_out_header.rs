use byteorder::{ByteOrder, LittleEndian};


/// USBTMC Device-Dependent Message Out Header
/// 
pub struct DevDepMsgOutHeader {

    // Total number of USBTMC message data bytes to be
    // sent in this USB transfer. This does not include the
    // number of bytes in this Bulk-OUT Header or
    // alignment bytes. Sent least significant byte first,
    // most significant byte last. TransferSize must be >
    // 0x00000000.
    transfer_size: u32,


    bm_transfer_attributes: u8,
}


impl DevDepMsgOutHeader 
{

    pub fn new(transfer_size: u32, eom: bool) -> DevDepMsgOutHeader {
    
        let mut bm_transfer_attributes = 0x00;
        if eom {
            bm_transfer_attributes = 0x01;
        }
    
        DevDepMsgOutHeader {
            transfer_size,
            bm_transfer_attributes,
        }
    }


    pub fn to_vec(&self) -> Vec<u8> {
        // Create header
        let mut hdr = vec![];
        hdr.append(&mut little_write_u32(self.transfer_size, 4));

        // bmTransferAttributes
        hdr.push(self.bm_transfer_attributes);

        // Reserved
        hdr.append(&mut vec![0x00; 3]);
        hdr
    }

}

/// Write u32 in little endian
/// 
fn little_write_u32(size: u32, len: u8) -> Vec<u8> {
    let mut buf = vec![0; len as usize];
    LittleEndian::write_u32(&mut buf, size);
    buf
}

