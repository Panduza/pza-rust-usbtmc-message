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

