use std::ptr;
use std::io::{Result};

/// Strip IP4/6 headers and leave only ICMP payload
///
/// Returns payload size
pub fn icmp_payload(buf: &mut [u8], size: isize) -> Result<usize> {
    // TODO: Handle invalid/unknown/empty buf data
    match buf[0] >> 4 {
        4 => {
            let header_size: usize = ((buf[0] & 0x0f) * 32 / 8) as usize;
            let payload_size: usize = (size - header_size as isize) as usize;

            unsafe {
                ptr::copy(&buf[header_size], &mut buf[0], payload_size);
            }

            Ok(payload_size)
        },
        6 => {
            let payload_size = (((buf[4] as u16) << 8) | buf[5] as u16) as usize;
            unsafe {
                // TODO: IPv6 header might be bigger than 40 bytes
                ptr::copy(&buf[40], &mut buf[0], payload_size);
            }
            Ok(payload_size)
        }
        // TODO: return InvalidData error
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::icmp_payload;

    #[test]
    fn ipv4_icmp_echo_reply() {
        let mut packet: [u8; 84] = [
            // ip
            0x45, 0x00, 0x00, 0x54, 0xd8, 0x11, 0x40, 0x00, 0x40, 0x01, 0x2d, 0xdc, 0xc0, 0xa8, 0x64, 0x03,
            0x08, 0x08, 0x08, 0x08,
            // icmp
            0x08, 0x00, 0x2b, 0xe8, 0x5f, 0xd3, 0x00, 0x01, 0x51, 0x28, 0xc8, 0x59, 0x00, 0x00, 0x00, 0x00,
            0x8c, 0xee, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
        ];
        let packet_len = packet.len() as isize;

        let size = icmp_payload(&mut packet, packet_len).unwrap();
        assert_eq!(size, 64);
        println!("{}", packet.len());
    }

    #[test]
    fn ipv6_icmp_echo_reply() {
        let mut packet: [u8; 104] = [
            // ip
            0x60, 0x09, 0x3f, 0xef, 0x00, 0x40, 0x3a, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            // icmp
            0x80, 0x00, 0xfe, 0x6c, 0x3b, 0xde, 0x00, 0x01, 0x5b, 0x1a, 0xc8, 0x59, 0x00, 0x00, 0x00, 0x00,
            0x5c, 0xf0, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
        ];
        let packet_len = packet.len() as isize;

        let size: usize = icmp_payload(&mut packet, packet_len).unwrap();

        assert_eq!(size, 64);
    }


}