
use super::Message;
use super::BulkInRequestMessage;


struct Sequencer {
    b_tag_index: u8,
    max_transfer_size: u32,
}

impl Sequencer {
    pub fn new(max_transfer_size: u32) -> Sequencer {
        Sequencer {
            b_tag_index: 0,
            max_transfer_size,
        }
    }

    fn next_b_tag(&mut self) -> u8 {
        self.b_tag_index = (self.b_tag_index % 255) + 1;
        self.b_tag_index
    }

    pub fn command_to_message_sequence<T: Into<String>>(&mut self, command: T) -> Vec<Message>
    {
        let cmd_obj = command.into();

        let mut sequence = Vec::new();
        

        // Requete BulkIn message
        sequence.push(
            Message::BulkInRequestMessage(
                // For now do not use term_char
                BulkInRequestMessage::new(self.next_b_tag(), self.max_transfer_size, None)
            )
        );

        sequence
    }

}



