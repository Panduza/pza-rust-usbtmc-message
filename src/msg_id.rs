#[derive(Copy, Clone)]
pub enum MsgID {
    DevDepMsgOut = 1,
    DevDepMsgIn = 2,
    VendorSpecificOut = 126,
    VendorSpecificIn = 127
}


impl MsgID {
    pub const DEV_DEP_MSG_OUT: MsgID = MsgID::DevDepMsgOut;

    pub const REQUEST_DEV_DEP_MSG_IN: MsgID = MsgID::DevDepMsgIn;
    pub const DEV_DEP_MSG_IN: MsgID = MsgID::DevDepMsgIn;

    pub const VENDOR_SPECIFIC_OUT: MsgID = MsgID::VendorSpecificOut;

    pub const REQUEST_VENDOR_SPECIFIC_IN: MsgID = MsgID::VendorSpecificIn;
    pub const VENDOR_SPECIFIC_IN: MsgID = MsgID::VendorSpecificIn;


    pub fn from_u8_array(value: u8) -> Option<MsgID> {
        match value {
            1 => Some(MsgID::DevDepMsgOut),
            2 => Some(MsgID::DevDepMsgIn),
            126 => Some(MsgID::VendorSpecificOut),
            127 => Some(MsgID::VendorSpecificIn),
            _ => None
        }
    }
}
