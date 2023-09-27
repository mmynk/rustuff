use netlink_packet_route::tc::{self, FqCodelQDiscStats};

use crate::tc::Classless;

#[derive(Clone, Debug, Default)]
pub struct FqCodel {
    target: u32,
    limit: u32,
    interval: u32,
    ecn: u32,
    flows: u32,
    quantum: u32,
    ce_threshold: u32,
    drop_batch_size: u32,
    memory_limit: u32,
}

impl FqCodel {
    pub fn new(fq_codel: &tc::nlas::FqCodel) -> Self {
        Self {
            target: fq_codel.target,
            limit: fq_codel.limit,
            interval: fq_codel.interval,
            ecn: fq_codel.ecn,
            flows: fq_codel.flows,
            quantum: fq_codel.quantum,
            ce_threshold: fq_codel.ce_threshold,
            drop_batch_size: fq_codel.drop_batch_size,
            memory_limit: fq_codel.memory_limit,
        }
    }
}


impl Into<Classless> for FqCodel {
    fn into(self) -> Classless {
        Classless::FqCodel(self)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FqCodelXStats {
    maxpacket: u32,
    drop_overlimit: u32,
    ecn_mark: u32,
    new_flow_count: u32,
    new_flows_len: u32,
    old_flows_len: u32,
    ce_mark: u32,
    memory_usage: u32,
    drop_overmemory: u32,
}

impl FqCodelXStats {
    pub fn new(xstats: &FqCodelQDiscStats) -> Self {
        Self {
            maxpacket: xstats.maxpacket,
            drop_overlimit: xstats.drop_overlimit,
            ecn_mark: xstats.ecn_mark,
            new_flow_count: xstats.new_flow_count,
            new_flows_len: xstats.new_flows_len,
            old_flows_len: xstats.old_flows_len,
            ce_mark: xstats.ce_mark,
            memory_usage: xstats.memory_usage,
            drop_overmemory: xstats.drop_overmemory,
        }
    }
}
