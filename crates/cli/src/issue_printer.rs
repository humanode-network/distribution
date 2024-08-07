//! Issue printers report issues by "printing" them.

use std::task::Poll;

use futures::Sink;
use humanode_distribution_resolver::resolve::ResolutionError;

/// A sink that prints results to the stderr.
pub struct Stderr;

impl Sink<ResolutionError> for Stderr {
    type Error = ();

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(
        self: std::pin::Pin<&mut Self>,
        item: ResolutionError,
    ) -> Result<(), Self::Error> {
        let ResolutionError { url, error } = item;
        eprintln!("An error occurred during resolution at {url}: {error}");
        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
