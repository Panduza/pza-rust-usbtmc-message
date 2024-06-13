use crate::MsgID;
use crate::Header;
use crate::DevDepMsgOutHeader;


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

