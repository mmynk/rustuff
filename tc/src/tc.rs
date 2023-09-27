use std::collections::BTreeMap;

use netlink_packet_route::{tc, tc::{Nla, Stats2, StatsBasic, StatsQueue}};

use crate::{netlink, errors::TcError, fq_codel::{FqCodel, FqCodelXStats}};

#[derive(Clone, Debug, Default)]
pub struct Stats {
    // Stats2::StatsBasic
    bytes: u64,
    packets: u32,

    // Stats2::StatsQueue
    qlen: u32,
    backlog: u32,
    drops: u32,
    requeues: u32,
    overlimits: u32,

    // XStats
    xstats: Option<XStats>,

    bps: u32,
    pps: u32,
}

#[derive(Clone, Debug)]
pub enum Classless {
    FqCodel(FqCodel),
}

#[derive(Clone, Debug)]
pub enum XStats {
    FqCodel(FqCodelXStats),
}

#[derive(Clone, Debug, Default)]
pub struct QDisc {
    handle: Option<u32>,
    parent: Option<u32>,
    kind: Option<String>,
    stats: Option<Stats>,
    // backlog: Option<Backlog>,
    qdisc: Option<Classless>,
}

fn parse_stats(qdisc: &mut QDisc, tc_stats: &tc::Stats) {
    let stats = qdisc.stats.get_or_insert(Stats::default());
    stats.bps = tc_stats.bps;
    stats.pps = tc_stats.pps;
}

fn parse_stats_basic(qdisc: &mut QDisc, stats_basic: &StatsBasic) {
    let stats = qdisc.stats.get_or_insert(Stats::default());
    stats.bytes = stats_basic.bytes;
    stats.packets = stats_basic.packets;
}

fn parse_stats_queue(qdisc: &mut QDisc, stats_queue: &StatsQueue) {
    let stats = qdisc.stats.get_or_insert(Stats::default());
    stats.qlen = stats_queue.qlen;
    stats.backlog = stats_queue.backlog;
    stats.drops = stats_queue.drops;
    stats.requeues = stats_queue.requeues;
    stats.overlimits = stats_queue.overlimits;
}

fn parse_qdisc(qdisc: &mut QDisc, q_disc: &tc::nlas::QDisc) {
    match q_disc {
        tc::QDisc::FqCodel(fq_codel) => {
            qdisc.qdisc = Some(FqCodel::new(fq_codel).into());
        },
    }
}

fn parse_xstats(qdisc: &mut QDisc, xstats: &tc::XStats) {
    let stats = qdisc.stats.get_or_insert(Stats::default());
    match xstats {
        tc::XStats::FqCodel(fq_codel) => {
            stats.xstats = {
                if fq_codel.type_ == tc::fq_codel::FqCodelXStatsType::QDiscStats {
                    Some(XStats::FqCodel(FqCodelXStats::new(&fq_codel.qdisc_stats.as_ref().unwrap())).into())
                } else {
                    None
                }
            }
        },
        _ => (),
    }
}

pub fn get_qdiscs() -> Result<BTreeMap<u32, Vec<QDisc>>, TcError> {
    let messages = netlink::netlink()?;
    let mut qdiscs = BTreeMap::new();

    for message in &messages {
        let mut qdisc = QDisc::default();

        for nla in &message.nlas {
            match nla {
                Nla::Kind(kind) => qdisc.kind = Some(kind.clone()),
                Nla::Stats2(stats) => {
                    for stat in stats {
                        match stat {
                            Stats2::StatsBasic(stat) => parse_stats_basic(&mut qdisc, stat),
                            Stats2::StatsQueue(stat) => parse_stats_queue(&mut qdisc, stat),
                            // TODO: parse Stats2::StatsApp
                            _ => (),
                        }
                    }
                },
                Nla::Stats(stats) => parse_stats(&mut qdisc, stats),
                Nla::QDisc(q_disc) => parse_qdisc(&mut qdisc, q_disc),
                Nla::XStats(xstats) => parse_xstats(&mut qdisc, xstats),
                _ => (),
            }
        }

        let dev = message.header.index as u32;
        qdiscs
            .entry(dev)
            .or_insert(Vec::new())
            .push(qdisc);
    }

    Ok(qdiscs)
}
