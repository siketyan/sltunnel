use sltunnel::rustls::internal::pemfile::{certs, rsa_private_keys};
use sltunnel::rustls::{Certificate, NoClientAuth, PrivateKey, ServerConfig};
use sltunnel::Server;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

fn load_certificates(path: &Path) -> std::io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid cert"))
}

fn load_private_keys(path: &Path) -> std::io::Result<Vec<PrivateKey>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid key"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args().into_iter().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: {} [bind_to] [upstream]", &args[0]);
        return Ok(());
    }

    let certificates = load_certificates(Path::new("./cert.pem"))?;
    let mut private_keys = load_private_keys(Path::new("./privkey.pem"))?;
    let mut server_config = ServerConfig::new(NoClientAuth::new());

    let num_certificates = certificates.len();
    let num_private_keys = private_keys.len();

    server_config.set_single_cert(certificates, private_keys.remove(0))?;

    println!(
        "Loaded {} certificates and {} private keys.",
        num_certificates, num_private_keys,
    );

    let bind_to = SocketAddr::from_str(&args[1])?;
    let upstream = SocketAddr::from_str(&args[2])?;
    let mut server = Server::start(bind_to, upstream, server_config).await?;

    println!("Listening on {} with upstream {}.", bind_to, upstream);

    loop {
        let (meta, pipes) = server.wait_for_session().await?;
        let (mut inbound, mut outbound) = pipes;

        tokio::spawn(async move { inbound.run().await });
        tokio::spawn(async move { outbound.run().await });

        println!(
            "Connection established: {} <-> {} (TLS).",
            upstream,
            meta.get_peer_addr(),
        );
    }
}
