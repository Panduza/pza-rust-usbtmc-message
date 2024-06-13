mod message;
mod bulk_in_message;
mod bulk_in_request_message;
mod bulk_out_request_message;


pub type Message = message::Message;
pub type BulkInMessage = bulk_in_message::BulkInMessage;
pub type BulkInRequestMessage = bulk_in_request_message::BulkInRequestMessage;
pub type BulkOutRequestMessage = bulk_out_request_message::BulkOutRequestMessage;

