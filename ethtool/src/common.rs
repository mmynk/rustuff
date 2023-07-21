pub const ETHTOOL_GENL_NAME: u8 = 0x14;
pub const ETHTOOL_GENL_VERSION: u8 = 1;

pub const ETHTOOL_A_HEADER_DEV_INDEX: u16 = 3;

// #[neli_enum(serialized_type = "u8")]
// pub enum EthtoolCmd {
//     // Userspace to kernel
//     GetStats = 32 as u8,

//     // Kernel to userspace
//     GetStatsReply = 33 as u8
// }

pub const ETHTOOL_MSG_STATS_GET: u16 = 32;
// pub const ETHTOOL_MSG_STATS_GET_REPLY: u16 = 33;
