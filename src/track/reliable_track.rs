use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;

use super::{PendingTrack, Track, TrackListener, ListenerBindError};
use sideway::ibverbs::queue_pair::{GenericQueuePair, QueuePair};
use sideway::rdmacm::communication_manager::{EventChannel, Identifier, PortSpace};

pub struct ReliableTrack<'a> {
    qp: GenericQueuePair<'a>,
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
}

impl Track for ReliableTrack<'_> {
    fn local_addr(&self) -> Option<SocketAddr> {
        Some(self.local_addr)
    }

    fn peer_addr(&self) -> Option<SocketAddr> {
        Some(self.peer_addr)
    }
}

pub struct PendingReliableTrack {}

impl PendingTrack for PendingReliableTrack {
    fn establish<'a>(self) -> Result<ReliableTrack<'a>, String> {
        todo!()
    }
}

pub struct ReliableTrackListener {
    channel: Option<EventChannel>,
    listen_id: Option<Arc<Identifier>>,
    local_addr: Option<SocketAddr>,
}

impl TrackListener for ReliableTrackListener {
    fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self, ListenerBindError> {
        let mut channel = EventChannel::new().unwrap();

        let cm_id = channel.create_id(PortSpace::Tcp).unwrap();

        let sock_addr = addr.to_socket_addrs().unwrap().next().unwrap();

        cm_id.bind_addr(sock_addr).unwrap();

        Ok(ReliableTrackListener{
            channel: Some(channel),
            listen_id: Some(cm_id),
            local_addr: Some(sock_addr),
        })
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        return self.local_addr;
    }

    fn accept(&self) -> Result<(PendingReliableTrack, SocketAddr), String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::track::TrackListener;

    use super::ReliableTrackListener;

    #[test]
    fn bind_address() {
        let _listener = ReliableTrackListener::bind(("127.0.0.1", 18515));
    }
}
