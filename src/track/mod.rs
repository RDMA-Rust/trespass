pub mod reliable_track;

use std::net::{SocketAddr, ToSocketAddrs};

#[derive(thiserror::Error, Debug)]
pub enum ListenerBindError {
    #[error("create event channel failed")]
    CreateChannel,
    #[error("create communication identifier failed")]
    CreateId,
    #[error("failed to bind address")]
    BindAddress,
}

/// Represents a basic RDMA communication element.
///
/// Track is the fundamental abstraction over different RDMA queue pair types, it could be:
/// - RC (Reliable Connected) queue pairs
/// - UD (Unreliable Datagram) queue pairs
/// - DCT (Dynamically Connected Transport) queue pairs
///
/// It serves as the foundation for higher-level Carrier abstractions that provide
/// specific communication patterns like streams or message-based communication.
pub trait Track {
    /// Returns the local socket address that this Track is bound to.
    fn local_addr(&self) -> Option<SocketAddr>;

    /// Returns the socket address of the remote peer this Track is connected to.
    ///
    /// For connection-oriented transports (RC, DCT), this returns the address of the
    /// connected peer. For connectionless transports (UD), this returns the last
    /// communicated peer address or an error if no communication has occurred.
    fn peer_addr(&self) -> Option<SocketAddr>;
}

/// Represents a pending RDMA connection that needs configuration before
/// it can be established as a full Track.
pub trait PendingTrack {
    /// Establishes the RDMA connection and returns a fully configured Track.
    fn establish<'a>(self) -> Result<impl Track, String>;
}

pub trait TrackListener {
    /// Creates a new `TrackListener` which will be bound to the specified
    /// address.
    ///
    /// The returned listener is ready for accepting connections.
    ///
    /// Binding with a port number of 0 will request that the OS assigns a port
    /// to this listener. The port allocated can be queried via the
    /// [`TrackListener::local_addr`] method.
    ///
    /// The address type can be any implementor of [`ToSocketAddrs`] trait. See
    /// its documentation for concrete examples.
    ///
    /// If `addr` yields multiple addresses, `bind` will be attempted with
    /// each of the addresses until one succeeds and returns the listener. If
    /// none of the addresses succeed in creating a listener, the error returned
    /// from the last attempt (the last address) is returned.
    fn bind<A: ToSocketAddrs>(addr: A) -> Result<impl TrackListener, ListenerBindError>;

    /// Accepts a new incoming connection, returning a PendingTrack.
    ///
    /// The PendingTrack must be configured with necessary RDMA parameters
    /// before it can be established into a full Track.
    fn accept(&self) -> Result<(impl PendingTrack, SocketAddr), String>;

    /// Returns the local socket address of this listener.
    fn local_addr(&self) -> Option<SocketAddr>;
}
