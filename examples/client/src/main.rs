use sltunnel::rustls::ClientConfig;
use sltunnel::Client;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

fn load_ca(config: &mut ClientConfig, path: &Path) -> std::io::Result<(usize, usize)> {
    config
        .root_store
        .add_pem_file(&mut BufReader::new(File::open(path)?))
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid key"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = args().into_iter().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: {} [bind_to] [upstream] [hostname]", &args[0]);
        return Ok(());
    }

    let mut client_config = ClientConfig::new();
    let (num_certificates, _) = load_ca(&mut client_config, Path::new("./ca.pem"))?;

    println!("Loaded {} certificates.", num_certificates);

    let hostname = &args[3];
    let bind_to = SocketAddr::from_str(&args[1])?;
    let upstream = SocketAddr::from_str(&args[2])?;
    let mut client = Client::start(hostname, bind_to, upstream, client_config).await?;

    println!(
        "Listening on {} with upstream {} ({}).",
        bind_to, hostname, upstream,
    );

    loop {
        let (meta, pipes) = client.wait_for_session().await?;
        let (mut inbound, mut outbound) = pipes;

        tokio::spawn(async move { inbound.run().await });
        tokio::spawn(async move { outbound.run().await });

        println!(
            "Connection established: {} (TLS) <-> {}.",
            upstream,
            meta.get_peer_addr(),
        );
    }
}
