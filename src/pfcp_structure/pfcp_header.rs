pub const PFCP_VER: u8 = 0b11100000;
pub const PFCP_S_FLAG: u8 = 0b00000001;
pub const PFCP_MP_FLAG: u8 = 0b00000010;
pub const PFCP_FO_FLAG: u8 = 0b00000100;


pub struct PfcpHeader {
    pub ver:      u8,     // Version (3 bits) + Spare (2 bits) + FO flag (1 bit) + MP flag (1 bit) + S flag (1 bit)
    pub flag_id:  bool,
    pub flag_mp:  bool,
    pub flag_fo:  bool,
    pub typ:      u8,     // Message type (8 bits)
    pub len:      u16,    // Message length (16 bits)
    pub id:       Option<u64>, // SEID (64 bits)
    pub seq:      u32,    // Sequence number (24 bits)
    pub mp:       u8,     // Message Priority (4 bits)
}


impl PfcpHeader {
    pub fn new(&self) -> Self {
        PfcpHeader {
            ver:        0,     // Version (3 bits) + Spare (2 bits) + FO flag (1 bit) + MP flag (1 bit) + S flag (1 bit)
            flag_id:    false,
            flag_mp:    false,
            flag_fo:    false,
            typ:        0,     // Message type (8 bits)
            len:        0,     // Message length (16 bits)
            id:         Some(0), // SEID (64 bits)
            seq:        0,      // Sequence number (24 bits) + Message Priority (4 bits) + Spare (4 bits)
            mp:         0,      // Message Priority (4 bits)
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.ver.to_be_bytes());
        buf.extend_from_slice(&self.typ.to_be_bytes());
        buf.extend_from_slice(&self.len.to_be_bytes());
        match self.id {
            Some(id) => buf.extend_from_slice(&id.to_be_bytes()),
            None => (),
        }
        buf.extend_from_slice(&self.seq.to_be_bytes());

        buf
    }

    pub fn decode(&self, buf: Vec<u8>) -> Self {
        let mut p : usize = 0;

        let ver = buf[p] >> 5;
        let flag_id: bool = if buf[p] & PFCP_S_FLAG > 0 {true} else {false};
        let flag_mp: bool = if buf[p] & PFCP_MP_FLAG > 0 {true} else {false};
        let flag_fo: bool = if buf[p] & PFCP_FO_FLAG > 0 {true} else {false};
        p += 1; //Length of Version field

        let typ = buf[p];
        p += 1; //Length of Type field

        let len = u16::from_be_bytes([buf[p], buf[p+1]]);
        p += 2; //Length of Length field

        let id: Option<u64> = if flag_id == false {
            None
        } else {
            let id = Some(u64::from_be_bytes([buf[p], buf[p+1], buf[p+2], buf[p+3], buf[p+4], buf[p+5], buf[p+6], buf[p+7]]));
            p += 8; //Length of SEID field
            id
        };

        let seq = u32::from_be_bytes([buf[p], buf[p+1], buf[p+2], 0]);
        p += 3; //Length of Sequence Number field

        let mp = if flag_mp == false {
            0
        } else {
            buf[15] & 0x0F
        };

        PfcpHeader {
            ver,
            flag_id,
            flag_mp,
            flag_fo,
            typ,
            len,
            id,
            seq,
            mp,
        }
    }
}