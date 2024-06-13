use crate::MsgID;
use crate::Header;
use crate::DevDepMsgInHeader;


/// USBTMC Device-Dependent Message In Header
/// 
/// This message must be send to the device to request a bulk in transfer.
/// 
pub struct BulkInRequestMessage {
    header: Header,
    bulk_in_header: DevDepMsgInHeader,
}

impl BulkInRequestMessage {
    pub fn new(b_tag: u8, transfer_size: u32, term_char: Option<u8>) -> BulkInRequestMessage {
        BulkInRequestMessage {
            header: Header::new(MsgID::DevDepMsgIn, b_tag),
            bulk_in_header: DevDepMsgInHeader::new(transfer_size, term_char),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = self.header.to_vec();
        vec.append(&mut self.bulk_in_header.to_vec());

        vec
    }
}



