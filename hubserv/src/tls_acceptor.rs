// use clap::ArgGroup;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use futures::future::TryFutureExt;
use rustls_acme::acme::ACME_TLS_ALPN_NAME;
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncWrite};
// use tokio_stream::StreamExt;
use async_rustls::rustls::Session;
use tokio::io;
use tokio_util::compat::Compat;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[derive(Clone)]
pub(crate) struct TlsAcceptor(pub(crate) async_rustls::TlsAcceptor);

impl<I: AsyncRead + AsyncWrite + Unpin + Send + 'static, S: Send + 'static>
    axum_server::accept::Accept<I, S> for TlsAcceptor
{
    type Stream = Compat<async_rustls::server::TlsStream<Compat<I>>>;
    type Service = S;
    type Future = BoxFuture<'static, io::Result<(Self::Stream, Self::Service)>>;

    fn accept(&self, stream: I, service: S) -> Self::Future {
        self.0
            .accept(stream.compat())
            .map_ok(move |tls| {
                let tls = tls.compat();
                if let Some(ACME_TLS_ALPN_NAME) = tls.get_ref().get_ref().1.get_alpn_protocol() {
                    log::info!("received TLS-ALPN-01 validation request");
                }
                (tls, service)
            })
            .boxed()
    }
}
