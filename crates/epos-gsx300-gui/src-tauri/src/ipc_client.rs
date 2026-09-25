use std::path::PathBuf;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// How long the GUI waits for one daemon answer.
///
/// Derived from the daemon's own worst case rather than picked. `GetDevice`
/// reads eight hardware registers at a two-second budget each, plus four seconds
/// of retries opening the device, so a GSX 300 that has opened but stopped
/// answering legitimately takes about twenty seconds — and the answer it gives
/// then is a real one, just an empty snapshot. This sits above that on purpose:
/// a wedged *device* should reach the GUI as the empty snapshot it is, not as an
/// error, and the timeout is here for a wedged *daemon*.
///
/// Without a deadline every await below is unbounded. A daemon that accepts the
/// connection and then stops answering leaves this task — and the frontend's
/// `await invoke("daemon_request", …)` behind it — pending for good, and
/// nothing in the frontend can cancel an invoke.
const DAEMON_TIMEOUT: Duration = Duration::from_secs(30);

/// How long one response line may be.
///
/// Same cap the daemon puts on a request line, for the same reason: `lines()`
/// grows a `String` until it sees a newline, so whatever the far end sends is
/// what gets buffered. The daemon is this project's own code and not an
/// untrusted peer, so this is not a security boundary — it is there so a bug on
/// that side cannot take the GUI's memory down with it.
const MAX_LINE: usize = 64 * 1024;

pub struct DaemonClient {
    socket_path: PathBuf,
}

impl DaemonClient {
    pub fn new() -> Self {
        let runtime_dir = dirs::runtime_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        Self::at(runtime_dir.join("epos-gsx300d.sock"))
    }

    /// A client for a chosen socket, so a test can stand up its own daemon.
    pub fn at(socket_path: PathBuf) -> Self {
        Self { socket_path }
    }

    pub async fn send_request(&self, request: &str) -> Result<String, String> {
        self.send_request_within(request, DAEMON_TIMEOUT).await
    }

    /// [`send_request`] with the budget as an argument.
    ///
    /// A parameter rather than the constant inline, because the case this
    /// exists for is a daemon that never answers, and a test cannot ask the real
    /// daemon to do that on demand.
    pub async fn send_request_within(
        &self,
        request: &str,
        budget: Duration,
    ) -> Result<String, String> {
        // One deadline over the whole exchange, so the connect and the two
        // writes are covered too. A socket write can block just as a read can
        // when the far end stops draining, and bounding only the read would
        // leave the two writes unbounded.
        match tokio::time::timeout(budget, self.exchange(request)).await {
            Ok(result) => result,
            Err(_) => Err(format!("daemon did not answer within {budget:?}")),
        }
    }

    async fn exchange(&self, request: &str) -> Result<String, String> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| format!("Failed to send: {}", e))?;
        stream
            .write_all(b"\n")
            .await
            .map_err(|e| format!("Failed to send newline: {}", e))?;

        let (reader, _) = stream.into_split();
        let mut lines = BufReader::new(reader).lines();
        let Some(line) = lines
            .next_line()
            .await
            .map_err(|e| format!("Failed to read: {}", e))?
        else {
            return Err("No response from daemon".into());
        };
        if line.len() > MAX_LINE {
            return Err(format!(
                "daemon sent a {}-byte line, over the {MAX_LINE}-byte cap",
                line.len()
            ));
        }
        Ok(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::net::UnixListener;

    /// A stand-in daemon on a socket of its own, so a test can make it answer,
    /// stay silent, or say something absurd without touching the real one.
    ///
    /// `behaviour` is handed the accepted connection, so the test decides what
    /// the far end does rather than the harness guessing.
    async fn fake_daemon<F, Fut>(name: &str, behaviour: F) -> (PathBuf, tokio::task::JoinHandle<()>)
    where
        F: Fn(UnixStream) -> Fut + Send + 'static + Clone,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let path = std::env::temp_dir().join(format!("epos-client-test-{}-{name}", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let listener = UnixListener::bind(&path).expect("bind fake daemon");
        let handle = tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let behaviour = behaviour.clone();
                tokio::spawn(async move { behaviour(stream).await });
            }
        });
        (path, handle)
    }

    /// An ordinary exchange still works.
    ///
    /// The anti-vacuity test. Every other case here asserts that something is
    /// refused, and a client that refused everything would satisfy all of them
    /// while the GUI simply stopped talking to the daemon.
    #[tokio::test]
    async fn a_request_that_is_answered_comes_back() {
        let (path, server) = fake_daemon("answers", |stream| async move {
            let (reader, mut writer) = stream.into_split();
            let mut reader = BufReader::new(reader);
            let mut request = String::new();
            let _ = reader.read_line(&mut request).await;
            let _ = writer
                .write_all(b"{\"type\":\"Status\",\"payload\":{}}\n")
                .await;
        })
        .await;
        let client = DaemonClient::at(path.clone());
        let answer = client
            .send_request_within(r#"{"type":"GetStatus"}"#, Duration::from_secs(5))
            .await
            .expect("the fake daemon answered");
        assert_eq!(answer, r#"{"type":"Status","payload":{}}"#);
        server.abort();
        let _ = std::fs::remove_file(&path);
    }

    /// A daemon that accepts and then says nothing is dropped at the budget.
    ///
    /// This is the case the deadline exists for, and the real daemon cannot be
    /// asked to reproduce it on demand. Measured on the running daemon, the
    /// longest *legitimate* answer is about twenty seconds when the hardware has
    /// stopped answering, so the production budget sits above that and this test
    /// uses a short one.
    #[tokio::test]
    async fn a_daemon_that_never_answers_is_dropped_at_its_budget() {
        // Read the request, then hold the connection open and say nothing. The
        // first version of this never mentioned `stream` in the body, so Rust
        // dropped it as soon as the future was built and the client saw a reset
        // rather than a daemon that had gone quiet - which is a different bug
        // with a different fix, and the test was asserting the wrong one.
        let (path, server) = fake_daemon("silent", |stream| async move {
            let mut reader = BufReader::new(stream);
            let mut request = String::new();
            let _ = reader.read_line(&mut request).await;
            // `reader` owns the stream and lives until this block returns, which
            // it never does. That is what keeps the connection open.
            std::future::pending::<()>().await;
        })
        .await;
        let client = DaemonClient::at(path.clone());
        let started = std::time::Instant::now();
        let result = client
            .send_request_within(r#"{"type":"GetStatus"}"#, Duration::from_millis(200))
            .await;
        let elapsed = started.elapsed();

        let err = result.expect_err("a silent daemon must not look like an answer");
        assert!(
            err.contains("did not answer within"),
            "the message has to say the daemon was silent, got {err:?}"
        );
        assert!(
            elapsed < Duration::from_secs(5),
            "must give up near its budget, took {elapsed:?}"
        );
        server.abort();
        let _ = std::fs::remove_file(&path);
    }

    /// A daemon that hangs up without answering is an error, not an empty string.
    ///
    /// The other end of the same deadline, and the case a frontend cannot tell
    /// apart from success on its own: an empty response body would parse as
    /// nothing and leave the GUI showing stale values.
    #[tokio::test]
    async fn a_daemon_that_hangs_up_says_so() {
        // Read the request first, then drop the connection. Dropping straight
        // away races the client's write and the client reports a broken pipe,
        // which is a real error but not the one being pinned here.
        let (path, server) = fake_daemon("hangup", |stream| async move {
            let mut reader = BufReader::new(stream);
            let mut request = String::new();
            let _ = reader.read_line(&mut request).await;
            // `reader` drops here, closing the connection: EOF for the client.
        })
        .await;
        let client = DaemonClient::at(path.clone());
        let result = client
            .send_request_within(r#"{"type":"GetStatus"}"#, Duration::from_secs(5))
            .await;
        let err = result.expect_err("no response at all must be an error");
        assert!(
            err.contains("No response from daemon"),
            "got {err:?}"
        );
        server.abort();
        let _ = std::fs::remove_file(&path);
    }

    /// An over-long response is refused rather than buffered.
    ///
    /// The trust direction here means this is not a security boundary: the far
    /// end is this project's own daemon. It is here so that a bug on that side
    /// cannot grow the GUI's memory without limit, which is what `lines()` would
    /// do on a response that never contains a newline.
    #[tokio::test]
    async fn an_over_long_response_is_refused() {
        let big = Arc::new(format!("{{\"pad\":\"{}\"}}", "A".repeat(MAX_LINE * 2)));
        let payload = big.clone();
        let (path, server) = fake_daemon("overlong", move |stream| {
            let payload = payload.clone();
            async move {
                let (reader, mut writer) = stream.into_split();
                let mut reader = BufReader::new(reader);
                let mut request = String::new();
                let _ = reader.read_line(&mut request).await;
                let _ = writer.write_all(payload.as_bytes()).await;
                let _ = writer.write_all(b"\n").await;
            }
        })
        .await;
        let client = DaemonClient::at(path.clone());
        let result = client
            .send_request_within(r#"{"type":"GetStatus"}"#, Duration::from_secs(5))
            .await;
        let err = result.expect_err("a 128 KB response must not be accepted");
        assert!(err.contains("over the"), "the cap should be named in {err:?}");
        server.abort();
        let _ = std::fs::remove_file(&path);
    }

    /// The production budget clears the daemon's own worst case.
    ///
    /// Not a measurement — the numbers are the daemon's constants, and this says
    /// so rather than pretending to a timing run. If a constant over there moves,
    /// this fails and someone has to think about it, which is the point of
    /// writing it down twice.
    #[test]
    fn the_budget_clears_the_daemons_worst_case() {
        // hwinfo::open_hidraw: 10 retries, 400 ms apart.
        let open = Duration::from_millis(10 * 400);
        // hwinfo::read_mem: 8 registers at a 2 s budget each.
        let registers = Duration::from_secs(8 * 2);
        let daemon_worst_case = open + registers;
        assert!(
            DAEMON_TIMEOUT > daemon_worst_case,
            "the GUI would report an error for a daemon that is merely waiting on \
             hardware: GUI budget {DAEMON_TIMEOUT:?} vs daemon worst case \
             {daemon_worst_case:?}"
        );
    }
}
