use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;

use crate::session::{create_session, Session};

type Upstream = TcpStream;
type Downstream = TlsStream<TcpStream>;
type ServerSession = Session<Upstream, Downstream>;

pub struct Server {
    upstream: SocketAddr,
    tcp_listener: TcpListener,
    tls_acceptor: TlsAcceptor,
}

impl Server {
    pub async fn start(
        bind_to: SocketAddr,
        upstream: SocketAddr,
        server_config: ServerConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            upstream,
            tcp_listener: TcpListener::bind(bind_to).await?,
            tls_acceptor: TlsAcceptor::from(Arc::new(server_config)),
        })
    }

    pub async fn wait_for_session(&mut self) -> Result<ServerSession, Box<dyn Error>> {
        let (downstream, peer_addr) = self.tcp_listener.accept().await?;
        let downstream = self.tls_acceptor.clone().accept(downstream).await?;
        let upstream = TcpStream::connect(self.upstream).await?;

        Ok(create_session(peer_addr, upstream, downstream))
    }
}
