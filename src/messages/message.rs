use super::bulk_in_request_message::BulkInRequestMessage;
use super::bulk_out_request_message::BulkOutRequestMessage;

pub enum Message {
    BulkOutRequestMessage(BulkOutRequestMessage),
    BulkInRequestMessage(BulkInRequestMessage),
}

impl Message {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Message::BulkOutRequestMessage(msg) => msg.to_vec(),
            Message::BulkInRequestMessage(msg) => msg.to_vec(),
        }
    }
}



#[test]
fn test_message() {
    let msg = Message::BulkInRequestMessage(BulkInRequestMessage::new(5, 32, None));
    let vec = msg.to_vec();
    assert_eq!(vec, vec![
        2, // Bulk-IN transfer
        5, // b_tag
        250, // b_tag_inverse
        0,
        32, 0, 0, 0, // transfer_size
        0, 0, 0, 0] );
}
