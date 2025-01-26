use crate::logger::Logger;
use crate::DevDepMsgInHeader;
use crate::Header;

/// USBTMC Device-Dependent Message In Header
///
/// This message is sent by the device to the host.
///
pub struct BulkInMessage {
    /// Header
    pub header: Header,
    /// USBTMC Device-Dependent Message In Header
    pub bulk_in_header: DevDepMsgInHeader,
    /// Payload
    pub payload: Vec<u8>,
}

impl BulkInMessage {
    pub fn from_u8_array(data: &[u8]) -> BulkInMessage {
        let payload_size = data
            .iter()
            .skip(12)
            .take_while(|c| **c != b'\n' && **c != b'\r')
            .count();

        

        //
        // Create trace logger
        let logger = Logger::new_for_crate();
        logger.trace(format!("!! payload_size={:?}", payload_size));
        
        
        let bulk_in_header = DevDepMsgInHeader::from_u8_array(&data[4..11]);
        logger.trace(format!("!! transfer_size={:?}", bulk_in_header.transfer_size()));
        

        let payload_sizeee = bulk_in_header.transfer_size() as usize + 12;
        let payload_array = &data[12..payload_sizeee];

        BulkInMessage {
            header: Header::from_u8_array(&data[0..4]).unwrap(),
            bulk_in_header: bulk_in_header,
            payload: payload_array.to_vec(),
        }
    }

    /// Getter for the bulk in header
    ///
    pub fn bulk_in_header(&self) -> &DevDepMsgInHeader {
        &self.bulk_in_header
    }

    /// Getter for the payload
    ///
    pub fn payload(&self) -> &Vec<u8> {
        &self.payload
    }

    /// Convert the payload to a string
    /// 
    /// Manage errors by return an error string
    ///
    pub fn payload_as_string(&self) -> String {
        match String::from_utf8(self.payload.clone()) {
            Ok(s) => s,
            Err(_) => "Cannot convert the payload in to string".to_string(),
        }
    }
}

#[test]
fn test_bulk_in_message() {
    let data = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x05, 0x06, 0x07, 0x08, 0x48, 0x45, 0x4C,
        0x4C, 0x4F, 13,
    ];

    let msg = BulkInMessage::from_u8_array(&data);

    assert_eq!(msg.payload_as_string(), "HELLO".to_string());
}
