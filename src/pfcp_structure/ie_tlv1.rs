pub struct tlv1 {
    t: u16,
    l: u16,
    v: u8,
}

// trait tlv1 {}
impl tlv1 {
    fn new(&self, t:u16) -> Self {
        tlv1 {
            t: t,
            l: 1,
            v: 0,
        }
    }

    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.t.to_be_bytes());
        buf.extend_from_slice(&self.l.to_be_bytes());
        buf.push(self.v);
        buf
    }

    fn decode(&self, buf: Vec<u8>) -> Self {
        let mut tlv = tlv1 {
            t: u16::from_be_bytes([buf[0], buf[1]]),
            l: u16::from_be_bytes([buf[2], buf[3]]),
            v: buf[4],
        };
        tlv
    }
}