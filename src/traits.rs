use tokio::io::{AsyncRead, AsyncWrite};

pub trait Readable: 'static + AsyncRead + Send {}
pub trait Writable: 'static + AsyncWrite + Send {}

impl<T: 'static + AsyncRead + Send> Readable for T {}
impl<T: 'static + AsyncWrite + Send> Writable for T {}
