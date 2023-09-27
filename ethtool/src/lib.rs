#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/ethtool_bindings.rs"));

mod errors;
mod ethtool;

use errors::EthtoolError;

pub fn ethtool_stats<T: ethtool::EthtoolReadable>(if_name: &str) -> Result<Vec<(String, u64)>, EthtoolError> {
    let ethtool = T::new(if_name)?;
    ethtool.stats()
}

#[cfg(test)]
mod tests {
    use crate::{ethtool_stats, ethtool::Ethtool};

    #[test]
    fn test_stats() {
        let result = ethtool_stats::<Ethtool>("ens5");
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_ne!(stats.len(), 0);
        assert!(stats.iter().any(|stat| stat.1 != 0));
    }
}
