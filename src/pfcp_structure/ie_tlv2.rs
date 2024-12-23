struct tlv2 {
    t: u16,
    l: u16,
    v: u16,
}

impl tlv2 {
    fn new(&self, t:u16) -> Self {
        tlv2 {
            t: t,
            l: 2,
            v: 0,
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.t.to_be_bytes());
        buf.extend_from_slice(&self.l.to_be_bytes());
        buf.extend_from_slice(&self.v.to_be_bytes());
        buf
    }

    fn decode(&self, buf: Vec<u8>) -> Self {
        let mut tlv = tlv2 {
            t: u16::from_be_bytes([buf[0], buf[1]]),
            l: u16::from_be_bytes([buf[2], buf[3]]),
            v: u16::from_be_bytes([buf[4], buf[5]]),
        };
        tlv
    }
}