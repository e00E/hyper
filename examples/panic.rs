use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

struct Io;

impl tokio::io::AsyncRead for Io {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        Poll::Pending
    }
}

impl tokio::io::AsyncWrite for Io {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context<'_>, _: &[u8]) -> Poll<Result<usize>> {
        Poll::Pending
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Pending
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Pending
    }
}

// cargo run --example panic --features full
#[tokio::main]
async fn main() {
    let (mut request_sender, mut connection) =
        hyper::client::conn::http1::handshake(Io).await.unwrap();
    let request = hyper::Request::builder().body("".to_string()).unwrap();
    let response = request_sender.send_request(request);
    futures_util::pin_mut!(response);

    let _ = futures_util::poll!(&mut connection);
    std::mem::drop(connection);
    // this line panics
    let _ = futures_util::poll!(&mut response);
}

/* same panic on 0.14
#[tokio::main]
async fn main_0_14() {
    let (mut request_sender, mut connection) = hyper::client::conn::handshake(Io).await.unwrap();
    let request = hyper::Request::builder().body("".into()).unwrap();
    let mut response = request_sender.send_request(request);

    let _ = futures_util::poll!(&mut connection);
    std::mem::drop(connection);
    // this line panics
    let _ = futures_util::poll!(&mut response);
}
*/
