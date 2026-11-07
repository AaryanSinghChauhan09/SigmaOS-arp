//! Arp model — manipulate ARP cache.

#[derive(Debug, Clone)]
pub struct ArpEntry {
    pub address: String,
    pub hw_type: String,
    pub hw_address: String,
    pub mask: String,
    pub flags: String,
    pub iface: String,
}

#[derive(Debug, Clone)]
pub struct Arp {
    pub show_all: bool,
    pub numeric: bool,
}

impl Default for Arp {
    fn default() -> Self { Self { show_all: false, numeric: false } }
}

impl Arp {
    pub fn new() -> Self { Self::default() }

    pub fn add(&self, ip: &str, mac: &str) -> ArpEntry {
        ArpEntry { address: ip.to_string(), hw_type: "ether".into(), hw_address: mac.to_string(), mask: "*".into(), flags: "C".into(), iface: "eth0".into() }
    }

    pub fn list(&self) -> Vec<ArpEntry> {
        vec![self.add("192.168.1.1", "00:11:22:33:44:55")]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_list() {
        let a = Arp::new();
        assert_eq!(a.list().len(), 1);
    }
}
