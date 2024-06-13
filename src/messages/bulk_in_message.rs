use crate::MsgID;
use crate::Header;
use crate::DevDepMsgInHeader;


/// USBTMC Device-Dependent Message In Header
/// 
pub struct BulkInMessage {
    header: Header,
    bulk_in_header: DevDepMsgInHeader,
}


impl BulkInMessage {

    pub fn from_vec(data: &Vec<u8>) -> BulkInMessage {
        
        BulkInMessage {
            header: Header::from_u8( &data[0..3] ).unwrap() ,
            bulk_in_header: DevDepMsgInHeader::from_u8( &data[4..11] ),
        }
    }


}

