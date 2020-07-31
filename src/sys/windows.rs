
const IPPROTO_ICMP: libc::c_int = 1;

const IP_TOS: libc::c_int = 1;

pub struct Socket {
    fd: libc::c_int,
    family: libc::c_int,
    peer: libc::sockaddr,
}