# Panduza Rust USBtmc Message

Helper to create and parse USBMC messages

This crate provides only the way to compose and decode USBtmc messages.
You need a USB lib to communicate with your device.
This crate is for developpers who want to create their own USBtmc crate with their own USB lib (like us in Panduza).


## Usage

```rust
use usbtmc_message::Sequencer;
use usbtmc_message::BulkInMessage;

fn main()
{
    // Create a sequencer with a max_sequence_length of 64 (depend on your device)
    let mut sequencer = Sequencer::new(64);

    // Create a message sequence from a command
    let seq = sequencer.command_to_message_sequence("*IDN?");

    // Send the sequence on the usb
    for i in 0..seq.len() {
        let message = seq[i].to_vec();
        // SEND TO USB
    }
    
    // RECEIVE FROM USB
    let data = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x05, 0x06, 0x07, 0x08,
        0x48, 0x45, 0x4C, 0x4C, 0x4F, 13
    ];

    // Parse the received data
    let msg = BulkInMessage::from_u8_array(&data);

    // Check the message
    assert_eq!( msg.payload_as_string(), "HELLO".to_string() );
}
```

## Specifications

`This crate does not currently implement every aspect of the specification`

This work is based on USB specification

```
Universal Serial Bus
Test and Measurement Class
Specification (USBTMC)
Revision 1.0
April 14, 2003 
```
