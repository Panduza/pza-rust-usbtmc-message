use crate::MsgID;
use crate::Header;
use crate::DevDepMsgInHeader;


/// USBTMC Device-Dependent Message In Header
/// 
/// This message is sent by the device to the host.
/// 
pub struct BulkInMessage {
    /// Header
    header: Header,
    /// USBTMC Device-Dependent Message In Header
    bulk_in_header: DevDepMsgInHeader,
    /// Payload
    payload: Vec<u8>,
}


impl BulkInMessage {


    pub fn from_u8_array(data: &[u8]) -> BulkInMessage 
    {
        let payload_size = data
            .iter()
            .skip(12)
            .take_while(|c| **c != b'\n' && **c != b'\r')
            .count();

        let payload_array = &data[12..payload_size + 12];

        BulkInMessage {
            header: Header::from_u8_array( &data[0..4] ).unwrap() ,
            bulk_in_header: DevDepMsgInHeader::from_u8_array( &data[4..11] ),
            payload: payload_array.to_vec(),
        }
    }

    /// 
    pub fn payload_as_string(&self) -> String {
        String::from_utf8(self.payload.clone()).unwrap()
    }

}



#[test]
fn test_bulk_in_message() {
    let data = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x05, 0x06, 0x07, 0x08,
        0x48, 0x45, 0x4C, 0x4C, 0x4F, 13
    ];

    let msg = BulkInMessage::from_u8_array(&data);

    assert_eq!( msg.payload_as_string(), "HELLO".to_string() );


}


