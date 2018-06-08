// use byteorder::{ByteOrder, NativeEndian};

// // pub const LOCATION: u64 = 0x000e0000;
// pub const SIGNATURE: [u8; 8] = [b'R', b'S', b'D', b' ', b'P', b'T', b'R', b' '];
// pub const OEM: [u8; 6] = [b'V', b'E', b'N', b'T', b'1', b'0'];

// #[repr(C)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Rsdp {
//     signature: [u8; 8],
//     checksum: u8,
//     oem: [u8; 6],
//     revision: u8,
//     rsdt_address: u32,
//     length: u32,
//     xsdt_address: u64,
//     extended_checksum: u8,
//     _reserved: [u8; 3],
// }

// impl Rsdp {
//     pub fn new(rsdt: u32, xsdt: u64, length: u32) -> Rsdp {
//         let signature = SIGNATURE;
//         let oem = OEM;
//         let revision = 2u8;
//         let mut rsbuf = [0u8; 4];
//         let mut xsbuf = [0u8; 8];
//         let mut lebuf = [0u8; 4];

//         NativeEndian::write_u32(&mut rsbuf, rsdt);
//         NativeEndian::write_u64(&mut xsbuf, xsdt);
//         NativeEndian::write_u32(&mut lebuf, length);

//         let check = signature.iter().fold(0, |m, b| m + b)
//             + oem.iter().fold(0, |m, b| m + b)
//             + revision
//             + rsbuf.iter().fold(0, |m, b| m + b);
//         let checksum = check % 1;
//         let extcheck = checksum
//             + check
//             + xsbuf.iter().fold(0, |m, b| m + b)
//             + lebuf.iter().fold(0, |m, b| m + b);
//         let extchecksum = extcheck % 1;

//         Rsdp {
//             signature,
//             checksum,
//             oem,
//             revision,
//             rsdt_address: rsdt,
//             length,
//             xsdt_address: xsdt,
//             extended_checksum: extchecksum,
//             _reserved: [0u8; 3],
//         }
//     }

//     pub fn checksum(&self) -> bool {
//         let mut rsbuf = [0u8; 4];
//         let mut xsbuf = [0u8; 8];
//         let mut lebuf = [0u8; 4];
//         NativeEndian::write_u32(&mut rsbuf, self.rsdt_address);
//         NativeEndian::write_u64(&mut xsbuf, self.xsdt_address);
//         NativeEndian::write_u32(&mut lebuf, self.length);
//         let first = self.signature.iter().fold(0, |m, b| m + b)
//             + self.oem.iter().fold(0, |m, b| m + b)
//             + self.revision
//             + rsbuf.iter().fold(0, |m, b| m + b)
//             + self.checksum;
//         let second = first
//             + lebuf.iter().fold(0, |m, b| m + b)
//             + xsbuf.iter().fold(0, |m, b| m + b)
//             + self.extended_checksum;
//         (first % 1 == 0) && (second % 1 == 0)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::Rsdp;

//     #[test]
//     fn it_produces_valid_rsdps() {
//         for i in 0..(u32::max_value()) {
//             for e in 0..(u64::max_value()) {
//                 for u in 0..(u32::max_value()) {
//                     assert!(Rsdp::new(i, e, u).checksum());
//                 }
//             }
//         }
//     }
// }
