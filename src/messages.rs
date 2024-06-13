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
    pub fn new(header: Header, bulk_out_header: DevDepMsgOutHeader, payload: Vec<u8>) -> BulkOutRequestMessage {
        BulkOutRequestMessage {
            header,
            bulk_out_header,
            payload,
        }
    }




    // pub fn to_vec(&self) -> Vec<u8> {
    //     let mut vec = self.header.to_vec();
    //     vec.append(&mut self.bulk_out_header.to_vec());
    //     vec.append(&mut self.payload.clone());

    //     vec
    // }
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



