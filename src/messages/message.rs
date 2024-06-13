use super::bulk_in_request_message::BulkInRequestMessage;
use super::bulk_out_request_message::BulkOutRequestMessage;

pub enum Message {
    BulkOutRequestMessage(BulkOutRequestMessage),
    BulkInRequestMessage(BulkInRequestMessage),
}

