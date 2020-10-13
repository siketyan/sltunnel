use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::client::TlsStream;
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::webpki::{DNSName, DNSNameRef};
use tokio_rustls::TlsConnector;

use crate::session::{create_session, Session};

type Upstream = TlsStream<TcpStream>;
type Downstream = TcpStream;
type ClientSession = Session<Upstream, Downstream>;

pub struct Client {
    hostname: DNSName,
    upstream: SocketAddr,
    tcp_listener: TcpListener,
    tls_connector: TlsConnector,
}

impl Client {
    pub async fn start(
        hostname: &str,
        bind_to: SocketAddr,
        upstream: SocketAddr,
        client_config: ClientConfig,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            upstream,
            hostname: DNSName::from(DNSNameRef::try_from_ascii_str(hostname)?),
            tcp_listener: TcpListener::bind(bind_to).await?,
            tls_connector: TlsConnector::from(Arc::new(client_config)),
        })
    }

    pub async fn wait_for_session(&mut self) -> Result<ClientSession, Box<dyn Error>> {
        let (downstream, peer_addr) = self.tcp_listener.accept().await?;
        let upstream = TcpStream::connect(self.upstream).await?;
        let upstream = self
            .tls_connector
            .connect(self.hostname.as_ref(), upstream)
            .await?;

        Ok(create_session(peer_addr, upstream, downstream))
    }
}
