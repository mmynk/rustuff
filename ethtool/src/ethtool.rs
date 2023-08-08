use std::str;

use nix::errno::Errno;
use nix::libc::ioctl;
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};

use crate::common::*;

#[derive(Debug)]
#[repr(C)]
struct StringSetInfo {
    cmd: u32,
    reserved: u32,
    mask: u32,
    data: usize,
}

#[derive(Debug)]
#[repr(C)]
struct GStrings {
    pub cmd: u32,
    pub string_set: u32,
    pub len: u32,
    pub data: [u8; MAX_GSTRINGS * ETH_GSTRING_LEN],
}

#[derive(Debug)]
#[repr(C)]
struct GStats {
    pub cmd: u32,
    pub len: u32,
    pub data: [u8; MAX_GSTRINGS * ETH_GSTRING_LEN],
}

#[derive(Debug)]
#[repr(C)]
struct IfReq {
    if_name: [u8; IFNAME_MAX_SIZE],
    if_data: usize,
}

fn _ioctl(fd: i32, if_name: &String, data: usize) -> Result<(), Errno> {
    let mut ifname = [0u8; IFNAME_MAX_SIZE];
    ifname
        .get_mut(..if_name.len())
        .unwrap()
        .copy_from_slice(if_name.as_bytes());
    let mut request = IfReq {
        if_name: ifname,
        if_data: data,
    };

    let exit_code = unsafe { ioctl(fd, nix::libc::SIOCETHTOOL, &mut request) };

    if exit_code != 0 {
        println!("code: {exit_code}, data: {:?}", request.if_data);
        return Err(Errno::from_i32(exit_code));
    }
    Ok(())
}

fn parse_features(data: [u8; MAX_GSTRINGS * ETH_GSTRING_LEN], length: usize) -> Vec<String> {
    let stats = (0..length as usize)
        .into_iter()
        .filter_map(|i| {
            let name_bytes = data
                .get(i * ETH_GSTRING_LEN..(i + 1) * ETH_GSTRING_LEN)
                .unwrap();

            name_bytes
                .iter()
                .position(|b| *b == 0)
                .and_then(|end| {
                    std::str::from_utf8(&name_bytes[..end])
                        .map_err(|e| println!("parse feature name failed: {}", e))
                        .ok()
                })
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
        })
        .collect();
    stats
}

// fn print_features(stats: &Vec<String>) {
//     println!("{:#<50}", "");
//     for name in stats {
//         println!("{:<50}", name);
//     }
//     println!("{:#<50}", "*");
// }

fn parse_values(data: [u8; MAX_GSTRINGS * ETH_GSTRING_LEN], length: usize) -> Vec<u64> {
    let mut values = Vec::with_capacity(length);
    let mut value_bytes = [0u8; 8];
    for i in 0..length {
        let offset = 8 * i;
        let data_slice = data.get(offset..offset + 8).unwrap();
        value_bytes.copy_from_slice(data_slice);

        values.push(u64::from_le_bytes(value_bytes));
    }

    values
}

fn print_stats(stats: &Vec<(String, u64)>) {
    println!("{:#<50}", "");
    for (name, value) in stats {
        println!("{:<50}: {:<10}", name, value);
    }
    println!("{:#<50}", "");
}

pub struct Ethtool {
    sock_fd: i32,
    if_name: String,
}

impl Ethtool {
    pub fn init(if_name: &str) -> Self {
        let fd = socket(
            AddressFamily::Inet,
            SockType::Datagram,
            SockFlag::empty(),
            None,
        )
        .expect("failed to open socket");

        Self {
            sock_fd: fd,
            if_name: String::from(if_name),
        }
    }

    /// Get the number of stats using ETHTOOL_GSSET_INFO command
    fn gsset_info(&self) -> Result<usize, Errno> {
        let mut sset_info = StringSetInfo {
            cmd: ETHTOOL_GSSET_INFO,
            reserved: 1,
            mask: 1 << ETH_SS_STATS,
            data: 0,
        };

        match _ioctl(
            self.sock_fd,
            &self.if_name,
            &mut sset_info as *mut StringSetInfo as usize,
        ) {
            Ok(_) => Ok(sset_info.data),
            Err(errno) => Err(errno),
        }
    }

    /// Get the feature names using ETHTOOL_GSTRINGS command
    fn gstrings(&self, length: usize) -> Result<Vec<String>, Errno> {
        let mut gstrings = GStrings {
            cmd: ETHTOOL_GSTRINGS,
            string_set: ETH_SS_STATS,
            len: length as u32,
            data: [0u8; MAX_GSTRINGS * ETH_GSTRING_LEN],
        };

        match _ioctl(
            self.sock_fd,
            &self.if_name,
            &mut gstrings as *mut GStrings as usize,
        ) {
            Ok(_) => Ok(parse_features(gstrings.data, length)),
            Err(errno) => Err(errno),
        }
    }

    /// Get the statistics for the features using EHTOOL_GSTATS command
    fn gstats(&self, features: &Vec<String>) -> Result<Vec<u64>, Errno> {
        let length = features.len();
        let mut gstats = GStats {
            cmd: ETHTOOL_GSTATS,
            len: features.len() as u32,
            data: [0u8; MAX_GSTRINGS * ETH_GSTRING_LEN],
        };

        match _ioctl(
            self.sock_fd,
            &self.if_name,
            &mut gstats as *mut GStats as usize,
        ) {
            Ok(_) => Ok(parse_values(gstats.data, length)),
            Err(errno) => Err(errno),
        }
    }

    pub fn stats(&self) -> Result<Vec<(String, u64)>, String> {
        println!("Fetching statistic for {}", &self.if_name);

        let length = match self.gsset_info() {
            Ok(length) => length,
            Err(errno) => {
                return Err(format!(
                    "failed to fetch number of stats using ETHTOOL_GSSET_INFO with errno={errno}"
                ))
            }
        };

        let features = match self.gstrings(length) {
            Ok(features) => features,
            Err(errno) => {
                return Err(format!(
                    "failed to fetch the stat names using ETHTOOL_GSTRINGS with errno={errno}"
                ))
            }
        };

        let values = match self.gstats(&features) {
            Ok(values) => values,
            Err(errno) => {
                return Err(format!(
                    "failed to fetch values of stats using ETHTOOL_GSTATS with errno={errno}"
                ))
            }
        };

        let final_stats = features.into_iter().zip(values).collect();
        print_stats(&final_stats);
        Ok(final_stats)
    }
}
