// pub const ETHTOOL_GENL_NAME: u8 = 0x14;
// pub const ETHTOOL_GENL_VERSION: u8 = 1;

// pub const ETHTOOL_MSG_STATS_GET: u16 = 32;

pub const ETHTOOL_GSSET_INFO: u32 = 0x37;
pub const ETHTOOL_GSTRINGS: u32 = 0x1b;
pub const ETHTOOL_GSTATS: u32 = 0x1d;
pub const ETH_SS_STATS: u32 = 0x1;
pub const ETH_GSTRING_LEN: usize = 32;

/// Maximum size of an interface name
pub const IFNAME_MAX_SIZE: usize = 16;

/// MAX_GSTRINGS maximum number of stats entries that ethtool can retrieve
pub const MAX_GSTRINGS: usize = 8192;
