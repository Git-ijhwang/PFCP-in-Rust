pub struct tlv4 {
    pub tlv_type: u8,
    pub tlv_length: u8,
    pub tlv_value: [u8; 4],
}


impl tlv4 {
    pub fn new(&self, t:u8) -> Self {
        tlv4 {
            tlv_type: t,
            tlv_length: 4,
            tlv_value: [0; 4],
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.tlv_type.to_be_bytes());
        buf.extend_from_slice(&self.tlv_length.to_be_bytes());
        buf.extend_from_slice(&self.tlv_value);
        buf
    }

    pub fn decode(&self, buf: Vec<u8>) -> Self {
        let mut tlv = tlv4 {
            tlv_type: buf[0],
            tlv_length: buf[1],
            tlv_value: [0; 4],
        };
        tlv.tlv_value.copy_from_slice(&buf[2..6]);
        tlv
    }
}