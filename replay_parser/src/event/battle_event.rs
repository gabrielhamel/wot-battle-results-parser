use enum_as_inner::EnumAsInner;
use enum_dispatch::enum_dispatch;

use super::{PacketParser, Unknown};
use crate::packet_stream::Packet;


pub enum BattleEventId {
    Generic,
    PositionUpdate = 10,
}

#[derive(Debug, EnumAsInner, Clone)]
#[enum_dispatch(ToPacket, EventPrinter)]
#[non_exhaustive]
pub enum BattleEvent {
    Unimplemented(Unknown),
}

impl BattleEvent {
    pub fn new(packet: &Packet) -> Self {
        match packet.get_type() {
            _ => Unknown::parse(packet),
        }
    }
}
