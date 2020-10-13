use std::net::SocketAddr;

use crate::pipe::{pipes, Pipes};
use crate::traits::{Readable, Writable};

pub struct SessionMeta {
    peer_addr: SocketAddr,
}

impl SessionMeta {
    pub fn get_peer_addr(&self) -> &SocketAddr {
        &self.peer_addr
    }
}

pub(crate) type Session<U, D> = (SessionMeta, Pipes<U, D>);

pub(crate) fn create_session<U, D>(
    peer_addr: SocketAddr,
    upstream: U,
    downstream: D,
) -> Session<U, D>
where
    U: Readable + Writable,
    D: Readable + Writable,
{
    (SessionMeta { peer_addr }, pipes(upstream, downstream))
}
