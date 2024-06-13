use super::MsgID;
use super::Header;
use super::DevDepMsgOutHeader;
use super::DevDepMsgInHeader;

pub enum Message {
    BulkOutRequestMessage(BulkOutRequestMessage),
    BulkInRequestMessage(BulkInRequestMessage),
}


pub struct BulkOutRequestMessage {
    header: Header,
    bulk_out_header: DevDepMsgOutHeader,
    payload: Vec<u8>,
}

impl BulkOutRequestMessage {
    
    pub fn new(b_tag: u8, block: &[u8], eom: bool) -> BulkOutRequestMessage {
        BulkOutRequestMessage {
            header: Header::new(MsgID::DevDepMsgOut, b_tag),
            bulk_out_header: DevDepMsgOutHeader::new(block.len() as u32, eom),
            payload : block.to_vec(),
        }
    }


    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = self.header.to_vec();
        vec.append(&mut self.bulk_out_header.to_vec());
        vec.append(&mut self.payload.clone());

        // align block on 4 bytes
        vec.append(&mut vec![0x00; (4 - (self.payload.len() % 4)) % 4]);

        vec
    }
}


/// USBTMC Device-Dependent Message In Header
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



