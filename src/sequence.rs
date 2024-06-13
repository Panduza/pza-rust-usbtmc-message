
use super::Message;



// Message

pub fn command_to_message_sequence<T: Into<String>>(command: T) -> Vec<Message>
{
    let cmd_obj = command.into();

    let mut sequence = Vec::new();
    // let mut command = command.to_string();
    // command.push('\n');
    // for c in command.chars() {
    //     sequence.push(c as u8);
    // }
    sequence
}



