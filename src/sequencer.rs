
use super::Message;
use super::BulkInRequestMessage;
use super::BulkOutRequestMessage;


pub struct Sequencer {
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
        let cmd: String = command.into();
        let cmd_bytes: &[u8] = cmd.as_bytes();

        let mut sequence = Vec::new();

        let mut num: usize = cmd_bytes.len();

        let mut eom = false;
        let mut offset = 0;
        while num > 0 {

            // check if end of message
            if num <= self.max_transfer_size as usize {
                eom = true;
            }
    
            // Prepare the block to send
            let block = &cmd_bytes[offset..(num - offset)];

            
            sequence.push(
                Message::BulkOutRequestMessage(
                    // For now do not use term_char
                    BulkOutRequestMessage::new(self.next_b_tag(), block, eom)
                )
            );
    
            offset += block.len();
            num = num - block.len();
        }


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



#[test]
fn test_simple_sequence() {

    let mut sequencer = Sequencer::new(64);

    let seq = sequencer.command_to_message_sequence("*IDN?");

    assert_eq!( seq.len(), 2 );
}


