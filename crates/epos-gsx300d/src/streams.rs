//! Keeping playback where the daemon meant to put it.
//!
//! Setting the default sink only steers streams opened *afterwards*. Measured on
//! this machine: audio the user was already listening to stayed on the speakers
//! through a default change, every time. So moving the user's audio is an
//! explicit operation, and this module is the part that decides *which* streams
//! to move and *when*.
//!
//! There are two directions, and both are pure functions over a stream listing:
//!
//! * **out** — a rescue, when the EQ chain dies and playback has to leave the
//!   anchor or it goes into a null sink and turns into silence;
//! * **back** — a return, once the chain is healthy again, so what the rescue
//!   moved is not left on the bypassed device.
//!
//! The decisions take the listing as data, so they are testable without a
//! headset, a PipeWire graph or a subprocess. The commands themselves stay in
//! `audio.rs`.
//!
//! ## The one hard constraint
//!
//! A PulseAudio sink input *only has an index to identify it* — there is no stable
//! server-side identifier, and the numbering is explicitly not persistent across
//! server restarts. So an index is the handle and nothing else, and it is only
//! meaningful within one PipeWire session. The ledger therefore records the
//! session cookie alongside the indices, and throws them away when the session
//! changes rather than acting on indices that now belong to other streams.
//!
//! Sink indices are recycled too, and even more freely: one measurement saw sink
//! 74 be the EPOS and, after a restart, the speakers. Nothing here ever caches a
//! sink index; callers resolve the name they need on every use.

use std::collections::{BTreeSet, HashMap};

// ─── Out: the rescue ─────────────────────────────────────────────

/// One entry of `pactl -f json list sink-inputs`, as far as a rescue cares.
///
/// `#[serde(default)]` throughout, so a pactl that grows or drops a field still
/// parses rather than turning a rescue into a parse error.
#[derive(Debug, serde::Deserialize)]
pub struct RescueStream {
    pub index: u32,
    pub sink: u32,
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

/// Node-name prefix for the streams the daemon owns itself.
///
/// A prefix rather than a fixed list, because the sidetone node only exists while
/// the sidetone is on, and a list would be wrong the moment that changed.
const OWN_NODE_PREFIX: &str = "epos-";

/// Is this one of the daemon's own streams?
///
/// `epos-eq-output` is the monitor that keeps the EQ chain draining, and it sits
/// on the anchor permanently — never corked, because the chain is always pulling
/// from it. A rescue that moved it would stop the chain outright: causing the
/// very outage the rescue exists to prevent, and then reporting that it had
/// saved the audio. It could not undo that either, since the rescue only runs
/// when the chain is already gone.
pub fn is_own_node(node_name: &str) -> bool {
    node_name.starts_with(OWN_NODE_PREFIX)
}

/// Which streams to pull off a dead anchor.
///
/// The daemon's own streams are excluded by [`is_own_node`], because moving them
/// would stop the chain the rescue is trying to save — see that function for the
/// full reason.
///
/// Corked streams are included. An application that is open but quiet will be on
/// the raw sink when it next speaks, which beats a per-application decision made
/// at the moment of the failure.
pub fn streams_to_rescue(streams: &[RescueStream], anchor_index: u32) -> Vec<u32> {
    streams
        .iter()
        .filter(|s| s.sink == anchor_index)
        .filter(|s| {
            !s.properties
                .get("node.name")
                .is_some_and(|n| is_own_node(n))
        })
        .map(|s| s.index)
        .collect()
}

// ─── Back: the return ────────────────────────────────────────────

/// The most `move-sink-input` commands one plan may carry.
///
/// Each command gets the full probe budget, so an unbounded batch could hold the
/// watchdog for minutes. The graph has tens of streams, not thousands, so the cap
/// is not expected to bite; when it does, the streams that did not fit stay put
/// and are named in the log rather than silently left behind.
pub const MAX_STREAM_MOVES: usize = 32;

/// Apply [`MAX_STREAM_MOVES`], reporting how many did not fit.
///
/// A separate function so the number left behind is something the caller can log.
/// Silently truncating would turn a bounded batch into a quiet data-loss bug.
pub fn capped(indices: Vec<u32>, cap: usize) -> (Vec<u32>, usize) {
    if indices.len() <= cap {
        return (indices, 0);
    }
    let dropped = indices.len() - cap;
    (indices.into_iter().take(cap).collect(), dropped)
}

/// A batch of `move-sink-input` commands, decided under the state lock and run
/// outside it.
///
/// Why it is split: one command per stream at a 5 s budget is minutes of waiting,
/// and holding the state lock across that freezes IPC, the config watcher and the
/// volume watcher. Deciding is cheap and quick; running the commands is not.
#[derive(Debug)]
pub struct StreamMovePlan {
    /// Where they are going: the raw device for a rescue, the EQ anchor for a
    /// return. A name, never an index, because sink indices are recycled.
    pub destination: String,
    pub indices: Vec<u32>,
    /// The PipeWire session the indices were read in. `None` means the cookie
    /// could not be read, in which case nothing may be recorded.
    pub cookie: Option<String>,
    pub purpose: MovePurpose,
}

/// Why a batch of streams is being moved, which is what the ledger does about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovePurpose {
    /// The EQ chain died and playback had to leave the anchor.
    Rescue,
    /// The chain is healthy again and the streams go back onto the anchor.
    Return,
    /// The user is leaving the EPOS for another device.
    ///
    /// Like [`MovePurpose::Return`] as far as the ledger is concerned — the streams
    /// are not on our hardware any more — but the log line differs, because they
    /// only come back if the user asks.
    Leave,
}

/// What running a plan actually achieved.
#[derive(Debug, Default)]
pub struct StreamMoveResult {
    pub moved: Vec<u32>,
    pub failed: Vec<u32>,
    /// The session afterwards. A change from the plan's means PipeWire restarted
    /// mid-plan, so the indices no longer mean what they meant.
    pub cookie: Option<String>,
}

/// Which streams to move off the EPOS when the user is leaving it.
///
/// The mirror of [`streams_to_rescue`], and it exists because of what was measured:
/// `pactl move-sink-input` *pins* a stream to its sink for good, so a rescue that
/// parked playback on the raw EPOS left it there after the user long-pressed away.
/// Changing the default sink does not touch it — that only steers streams opened
/// afterwards — so the EPOS went on playing the user's audio while the speakers
/// played the same thing. Measured: a Firefox stream sitting on the EPOS DAC,
/// uncorked, with the default sink on the speakers, and the EPOS DAC carrying a
/// *louder* signal than the speakers did.
///
/// So leaving takes every user stream off both of our sinks, not only the ones the
/// ledger happens to know about. The ledger lives in memory and a daemon restart
/// used to empty it, which is exactly when a pinned stream became untrackable.
pub fn streams_to_leave(entries: &[RescueStream], our_sink_indices: &[u32]) -> Vec<u32> {
    entries
        .iter()
        .filter(|s| our_sink_indices.contains(&s.sink))
        .filter(|s| {
            !s.properties
                .get("node.name")
                .is_some_and(|n| is_own_node(n))
        })
        .map(|s| s.index)
        .collect()
}

/// The line to log after a batch of streams was moved.
///
/// A pure function because the wording is the whole point. Leaving the EPOS and
/// returning to the EQ both forget the streams in the ledger, and when they
/// shared one arm the log claimed audio was "back on the EQ anchor" at the
/// moment it had just been moved to the user's speakers. Found by pressing the
/// button for the first time since that code shipped.
pub fn moved_message(purpose: MovePurpose, count: usize, destination: &str) -> String {
    match purpose {
        MovePurpose::Leave => {
            format!("{count} stream(s) moved off the EPOS to {destination}")
        }
        MovePurpose::Return | MovePurpose::Rescue => {
            format!("{count} stream(s) are back on the EQ anchor")
        }
    }
}

/// One stream's sink, as resolved by the caller from a listing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamOnSink {
    pub index: u32,
    pub sink_index: u32,
}

/// Parse `pactl list short sink-inputs` into (index, sink) pairs.
///
/// Column 2 is the sink, not the client. Measured on this machine, a stream
/// sitting on sink 33 lists as `11958  33  -  PipeWire  float32le 2ch 48000Hz`,
/// and an earlier version of this compared the second column to a sink *name*,
/// matched nothing, and reported a rescue that had moved nothing.
pub fn parse_stream_sinks(listing: &str) -> Vec<StreamOnSink> {
    listing
        .lines()
        .filter_map(|line| {
            let mut f = line.split_whitespace();
            let index = f.next()?.parse::<u32>().ok()?;
            let sink_index = f.next()?.parse::<u32>().ok()?;
            Some(StreamOnSink { index, sink_index })
        })
        .collect()
}

/// The streams to bring back onto the anchor, from those still parked on
/// `raw_index`.
///
/// Two filters, and both matter:
///
/// * **still alive** — an index whose stream has gone must not be moved. Acting
///   on it would either fail or, if the index has since been handed to another
///   stream, move somebody else's audio.
/// * **still on raw** — a stream the user has since moved somewhere else is
///   theirs now. Putting it back would overrule a choice made after ours.
///
/// The caller keeps ownership of the set; this only reads it, so a caller that
/// fails to move some streams can retry them without losing the ones that worked.
pub fn streams_to_return(
    rescued: &BTreeSet<u32>,
    present: &[StreamOnSink],
    raw_index: u32,
) -> Vec<u32> {
    present
        .iter()
        .filter(|s| s.sink_index == raw_index && rescued.contains(&s.index))
        .map(|s| s.index)
        .collect()
}

/// The session cookie from `pactl info`.
///
/// Parsed rather than grepped by the caller, so "there is no cookie" is a value
/// the type can express and the decision to discard indices can be made on it.
pub fn session_cookie(pactl_info: &str) -> Option<&str> {
    pactl_info.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("Cookie:")?;
        let value = rest.trim();
        (!value.is_empty()).then_some(value)
    })
}

/// Why a finished plan's indices must not be remembered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscardReason {
    /// No command succeeded, so there is nothing to remember.
    NothingMoved,
    /// The session changed while the commands ran.
    SessionChanged,
    /// No session cookie was available, so the indices cannot be tied to one.
    NoCookie,
}

/// What a finished plan means for the ledger.
#[derive(Debug, PartialEq, Eq)]
pub enum RecordAction {
    /// Do not remember these indices, for this reason.
    Discard(DiscardReason),
    /// Remember these as rescued, waiting to go back.
    Rescue(Vec<u32>),
    /// Forget these as rescued: they are home.
    Return(Vec<u32>),
}

/// Decide what a finished plan means for the ledger.
///
/// Three ways a move must not be remembered, and they are different failures:
///
/// * the session changed while the commands ran, so the numbers now belong to
///   whatever took their place — and the moves themselves may have hit the wrong
///   streams, which no check here can undo;
/// * no cookie was available at all, so the indices cannot be tied to a session
///   and the return path would refuse them anyway;
/// * nothing moved.
///
/// The first two are the difference between an outage the user can see and
/// somebody else's audio being moved on a later poll, so each is named instead of
/// being folded into a bool. Order matters: a changed session is reported as
/// that, not as a missing cookie.
pub fn record_action(
    plan_cookie: Option<&str>,
    after_cookie: Option<&str>,
    moved: &[u32],
    purpose: MovePurpose,
) -> RecordAction {
    if moved.is_empty() {
        return RecordAction::Discard(DiscardReason::NothingMoved);
    }
    if plan_cookie != after_cookie {
        return RecordAction::Discard(DiscardReason::SessionChanged);
    }
    if plan_cookie.is_none() {
        return RecordAction::Discard(DiscardReason::NoCookie);
    }
    match purpose {
        MovePurpose::Rescue => RecordAction::Rescue(moved.to_vec()),
        // Leaving is a return as far as the ledger is concerned: the streams are
        // no longer on our hardware, so there is nothing left to bring back.
        MovePurpose::Return | MovePurpose::Leave => RecordAction::Return(moved.to_vec()),
    }
}

/// What the daemon remembers about streams it moved, and whether it may still act.
#[derive(Debug, Default)]
pub struct StreamLedger {
    rescued: BTreeSet<u32>,
    /// The PipeWire session those indices belong to.
    cookie: Option<String>,
    /// The largest ledger worth keeping. A set that only grows is a leak, and the
    /// graph has tens of streams, not thousands.
    cap: usize,
}

impl StreamLedger {
    pub fn new(cap: usize) -> Self {
        Self {
            rescued: BTreeSet::new(),
            cookie: None,
            cap,
        }
    }

    /// Has the PipeWire session changed, invalidating every index we hold?
    ///
    /// A changed cookie means the indices name different streams now, so they are
    /// discarded rather than used. Said out loud because the alternative is moving
    /// a stranger's audio.
    pub fn session_changed(&mut self, cookie: Option<&str>) -> bool {
        let changed = self.cookie.is_some() && self.cookie.as_deref() != cookie;
        if changed {
            self.rescued.clear();
        }
        self.cookie = cookie.map(str::to_string);
        changed
    }

    /// Is this the session the recorded indices belong to?
    ///
    /// False when we never got a cookie. Indices recorded without one cannot be
    /// shown to belong to the current session, so the return path refuses to act
    /// on them: leaving audio on the bypassed device is recoverable, moving
    /// somebody else's stream is not.
    pub fn same_session(&self, cookie: Option<&str>) -> bool {
        self.cookie.is_some() && self.cookie.as_deref() == cookie
    }

    /// Record streams the daemon has just moved, for this session.
    pub fn note_rescued(&mut self, indices: &[u32], cookie: Option<&str>) {
        if self.cookie.as_deref() != cookie {
            // A different session: whatever we held was never ours to act on.
            self.rescued.clear();
            self.cookie = cookie.map(str::to_string);
        }
        for i in indices {
            if self.rescued.len() >= self.cap {
                break;
            }
            self.rescued.insert(*i);
        }
    }

    /// Keep only the streams still parked on `raw_index`, and return them.
    ///
    /// This is the plan and the cleanup at once, and the cleanup is the point. A
    /// recorded stream that is no longer on raw has either come home by itself —
    /// changing the default sink does drag the streams that follow it, which is
    /// how most of them returned in the measured outage — or been moved by the
    /// user. Both mean it is not ours to move.
    ///
    /// Forgetting only the streams that had *vanished* left the ledger
    /// permanently non-empty after an outage: it kept the route decision waiting
    /// for a return that could never be planned again, because the streams it was
    /// waiting for were already home. That was found by measurement, not by
    /// reading: the first live outage ended with every stream back on the anchor
    /// and no completion line ever printed, because the last few had followed
    /// the default home on their own.
    pub fn retain_parked(&mut self, present: &[StreamOnSink], raw_index: u32) -> Vec<u32> {
        let keep: BTreeSet<u32> = streams_to_return(&self.rescued, present, raw_index)
            .into_iter()
            .collect();
        self.rescued.retain(|i| keep.contains(i));
        self.rescued.iter().copied().collect()
    }

    /// Forget a stream the daemon has successfully brought back.
    pub fn note_returned(&mut self, indices: &[u32]) {
        for i in indices {
            self.rescued.remove(i);
        }
    }

    /// The ledger as text, or `None` when there is nothing to remember.
    ///
    /// Text rather than a live handle so the caller decides where it lives. It has
    /// to outlive the process: a daemon restart does not restart PipeWire, so the
    /// cookie and the indices stay meaningful across one, and throwing them away
    /// is what let a pinned stream become untrackable and therefore permanent.
    pub fn snapshot(&self) -> Option<String> {
        if self.rescued.is_empty() {
            return None;
        }
        let indices: Vec<u32> = self.rescued.iter().copied().collect();
        let file = serde_json::json!({ "cookie": self.cookie, "indices": indices });
        serde_json::to_string(&file).ok()
    }

    /// Load a snapshot, keeping it only if it belongs to the current session.
    ///
    /// The session check is here rather than at the point of use so a stale file
    /// cannot sit in the ledger looking like something to act on. Returns whether
    /// anything was restored.
    pub fn restore(&mut self, text: &str, current_cookie: Option<&str>) -> bool {
        let Ok(file) = serde_json::from_str::<serde_json::Value>(text) else {
            return false;
        };
        let Some(cookie) = file.get("cookie").and_then(|c| c.as_str()) else {
            return false;
        };
        let empty: Vec<u32> = Vec::new();
        let indices: Vec<u32> = file
            .get("indices")
            .and_then(|i| i.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect())
            .unwrap_or(empty);
        if indices.is_empty() || Some(cookie) != current_cookie {
            return false;
        }
        self.rescued = indices.into_iter().take(self.cap).collect();
        self.cookie = Some(cookie.to_string());
        true
    }

    /// How many streams are waiting, for the log line and the route decision.
    pub fn len(&self) -> usize {
        self.rescued.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rescued.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn listing() -> Vec<StreamOnSink> {
        vec![
            StreamOnSink {
                index: 38,
                sink_index: 74,
            },
            StreamOnSink {
                index: 168,
                sink_index: 71,
            },
            StreamOnSink {
                index: 2628,
                sink_index: 74,
            },
        ]
    }

    // ─── The rescue ─────────────────────────────────────────────────

    /// The daemon's own stream on the anchor is the monitor that keeps the chain
    /// draining. Moving it stops the chain — which is the outage the rescue exists
    /// to prevent, and by then the rescue cannot undo what it did.
    #[test]
    fn the_rescue_never_moves_the_daemons_own_stream() {
        // Shapes taken from `pactl -f json list sink-inputs` on this machine.
        let json = r#"[
          {"index":38,"sink":33,"corked":false,"properties":{"node.name":"output.games_sink"}},
          {"index":4835,"sink":33,"corked":false,"properties":{"node.name":"epos-eq-output"}},
          {"index":168,"sink":33,"corked":false,"properties":{"node.name":"epos-sidetone-output"}}
        ]"#;
        let streams: Vec<RescueStream> = serde_json::from_str(json).expect("fixture parses");
        assert_eq!(
            streams_to_rescue(&streams, 33),
            vec![38],
            "the EQ monitor is what keeps the chain alive; it must not be moved"
        );
    }

    /// A stream with no node name at all is still somebody's audio.
    #[test]
    fn a_nameless_stream_on_the_anchor_is_still_rescued() {
        let streams: Vec<RescueStream> =
            serde_json::from_str(r#"[{"index":7,"sink":33}]"#).expect("fixture parses");
        assert_eq!(streams_to_rescue(&streams, 33), vec![7]);
    }

    /// Streams on other sinks are not ours to move.
    #[test]
    fn streams_elsewhere_are_left_alone_by_the_rescue() {
        let streams: Vec<RescueStream> = serde_json::from_str(
            r#"[{"index":7,"sink":74,"properties":{"node.name":"Firefox"}}]"#,
        )
        .expect("fixture parses");
        assert!(streams_to_rescue(&streams, 33).is_empty());
    }

    /// The exclusion is by prefix, so a future daemon node is covered by the
    /// same rule rather than needing to be added to a list.
    #[test]
    fn ownership_is_a_prefix_not_a_list() {
        assert!(is_own_node("epos-eq-output"));
        assert!(is_own_node("epos-sidetone-output"));
        assert!(is_own_node("epos-something-added-later"));
        assert!(!is_own_node("output.games_sink"));
        assert!(!is_own_node("Firefox"));
    }

    // ─── The command cap ────────────────────────────────────────────

    /// Under the cap, nothing is dropped and nothing is reported.
    #[test]
    fn a_batch_within_the_cap_is_untouched() {
        assert_eq!(capped(vec![1, 2, 3], 4), (vec![1, 2, 3], 0));
    }

    /// Over the cap, the overflow is counted rather than vanishing. A silent
    /// truncation would turn a bounded batch into quiet data loss.
    #[test]
    fn an_oversized_batch_says_how_many_were_left_behind() {
        let (kept, dropped) = capped(vec![1, 2, 3, 4, 5], 3);
        assert_eq!(kept, vec![1, 2, 3]);
        assert_eq!(dropped, 2, "the two that did not fit must be reported");
    }

    /// The cap the daemon ships with is a number, not an aspiration: it is the
    /// difference between one poll and a watchdog that cannot keep up.
    #[test]
    fn the_shipped_cap_is_bounded() {
        assert!(MAX_STREAM_MOVES > 0 && MAX_STREAM_MOVES <= 64);
    }

    // ─── Reading the short listing ──────────────────────────────────

    /// Column 2 is the sink. This is the shape that made an earlier rescue match
    /// nothing: it compared the second column against a sink name.
    #[test]
    fn the_short_listing_gives_index_and_sink() {
        // Measured from `pactl list short sink-inputs` on this machine.
        let listing = "37\t1374\t-\tPipeWire\tfloat32le 2ch 48000Hz\n\
                       8602\t4786\t-\tPipeWire\tfloat32le 2ch 48000Hz\n\
                       11958\t33\t-\tPipeWire\tfloat32le 2ch 48000Hz\n";
        assert_eq!(
            parse_stream_sinks(listing),
            vec![
                StreamOnSink { index: 37, sink_index: 1374 },
                StreamOnSink { index: 8602, sink_index: 4786 },
                StreamOnSink { index: 11958, sink_index: 33 },
            ]
        );
    }

    /// A truncated or error line must not become a stream with index 0, which is
    /// a real index and would be moved like any other.
    #[test]
    fn an_unparsable_line_is_skipped_not_zeroed() {
        let listing = "garbage\n7\t33\t-\tPipeWire\tfloat32le 2ch 48000Hz\n";
        assert_eq!(
            parse_stream_sinks(listing),
            vec![StreamOnSink { index: 7, sink_index: 33 }]
        );
        assert!(parse_stream_sinks("").is_empty());
    }

    // ─── The return decision ────────────────────────────────────────

    /// The normal case: what we parked on raw comes back.
    #[test]
    fn a_rescued_stream_still_on_raw_comes_back() {
        let rescued: BTreeSet<u32> = [2628].into_iter().collect();
        assert_eq!(streams_to_return(&rescued, &listing(), 74), vec![2628]);
    }

    /// A stream that has gone must not be moved. If its index has since been
    /// handed to another stream, moving it would move somebody else's audio.
    #[test]
    fn a_stream_that_is_gone_is_not_returned() {
        let rescued: BTreeSet<u32> = [9999].into_iter().collect();
        assert!(streams_to_return(&rescued, &listing(), 74).is_empty());
    }

    /// A stream the user moved somewhere else is theirs now.
    #[test]
    fn a_stream_the_user_moved_away_is_not_returned() {
        let rescued: BTreeSet<u32> = [168].into_iter().collect();
        assert!(
            streams_to_return(&rescued, &listing(), 74).is_empty(),
            "168 is on the EPOS already, not on raw"
        );
    }

    /// Nothing rescued, nothing to do.
    #[test]
    fn nothing_rescued_returns_nothing() {
        assert!(streams_to_return(&BTreeSet::new(), &listing(), 74).is_empty());
    }

    // ─── The session cookie ─────────────────────────────────────────

    /// The cookie is read out of `pactl info` rather than grepped, so "no cookie"
    /// is a value and not an empty string.
    #[test]
    fn the_session_cookie_is_read_from_pactl_info() {
        let info = "Server Name: PulseAudio (on PipeWire 1.6.9)\n\
                    Server Version: 16.1\n\
                    Cookie: 1869336179\n\
                    Default Sink: speakers\n";
        assert_eq!(session_cookie(info), Some("1869336179"));
    }

    /// A `pactl` that prints no cookie must not read as an empty one.
    #[test]
    fn no_cookie_reads_as_none() {
        assert_eq!(session_cookie("Server Name: PulseAudio\n"), None);
        assert_eq!(session_cookie("Cookie:\n"), None);
    }

    // ─── Surviving a restart ────────────────────────────────────────

    /// What the daemon forgets must also survive its own restart, or a pinned
    /// stream becomes permanent: the rescue moved it, the daemon restarted to
    /// deploy a fix, and from then on nothing knew it was ours to move.
    #[test]
    fn a_snapshot_round_trips_and_needs_the_same_session() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[8792, 4714], Some("session-a"));
        let text = ledger.snapshot().expect("a non-empty ledger has a snapshot");

        let mut restarted = StreamLedger::new(64);
        assert!(
            restarted.restore(&text, Some("session-a")),
            "the same PipeWire session means the indices are still ours"
        );
        assert_eq!(restarted.len(), 2);
    }

    /// A different session refuses the file outright, so a stale ledger never sits
    /// in memory looking like something to act on.
    #[test]
    fn a_snapshot_from_another_session_is_refused() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[8792], Some("session-a"));
        let text = ledger.snapshot().expect("snapshot");

        let mut restarted = StreamLedger::new(64);
        assert!(!restarted.restore(&text, Some("session-b")));
        assert!(!restarted.restore(&text, None), "no cookie means no proof");
        assert!(restarted.is_empty());
    }

    /// A file that is not ours, or is damaged, changes nothing.
    #[test]
    fn a_damaged_snapshot_is_ignored() {
        let mut ledger = StreamLedger::new(64);
        assert!(!ledger.restore("not json", Some("c")));
        assert!(!ledger.restore(r#"{"cookie":"c"}"#, Some("c")));
        assert!(!ledger.restore(r#"{"cookie":"c","indices":[]}"#, Some("c")));
        assert!(ledger.is_empty());
    }

    /// An empty ledger has no file, so leaving no trace is the normal case.
    #[test]
    fn an_empty_ledger_has_no_snapshot() {
        assert!(StreamLedger::new(64).snapshot().is_none());
    }

    // ─── Leaving the EPOS ──────────────────────────────────────────

    fn leave_entries() -> Vec<RescueStream> {
        serde_json::from_str(
            r#"[
              {"index":8792,"sink":71,"properties":{"node.name":"Firefox"}},
              {"index":32224,"sink":71,"properties":{"node.name":"epos-eq-output"}},
              {"index":168,"sink":71,"properties":{"node.name":"epos-sidetone-output"}},
              {"index":36543,"sink":33,"properties":{"node.name":"paplay"}},
              {"index":33570,"sink":74,"properties":{"node.name":"Firefox"}}
            ]"#,
        )
        .expect("fixture parses")
    }

    /// The measured case: leaving takes the user's audio off the EPOS, or it keeps
    /// playing there while the speakers play it too.
    #[test]
    fn leaving_takes_user_streams_off_both_of_our_sinks() {
        // 71 is the raw EPOS, 33 the EQ anchor, 74 the user's speakers.
        assert_eq!(
            streams_to_leave(&leave_entries(), &[71, 33]),
            vec![8792, 36543],
            "a stream pinned on the EPOS keeps playing there after the default moves"
        );
    }

    /// The daemon's own streams stay: they are the chain being left, not audio.
    #[test]
    fn leaving_does_not_take_the_daemons_own_streams() {
        assert_eq!(streams_to_leave(&leave_entries(), &[71]), vec![8792]);
    }

    /// A stream already on the user's own device is not ours to move.
    #[test]
    fn leaving_leaves_other_devices_alone() {
        let moved = streams_to_leave(&leave_entries(), &[33]);
        assert!(moved.contains(&36543));
        assert!(!moved.contains(&33570), "33570 is already on the speakers");
    }

    // ─── The log wording ───────────────────────────────────────────

    /// Leaving says where the audio went. It must not say "back on the EQ
    /// anchor": that is the opposite destination, and the first live press
    /// produced exactly that false line.
    #[test]
    fn leaving_names_the_device_the_audio_went_to() {
        let line = moved_message(MovePurpose::Leave, 3, "alsa_output.speakers");
        assert!(line.contains("alsa_output.speakers"), "got: {line}");
        assert!(!line.contains("anchor"), "got: {line}");
    }

    /// Returning does name the anchor, which is where they went.
    #[test]
    fn returning_names_the_anchor() {
        let line = moved_message(MovePurpose::Return, 3, "epos-eq-processed");
        assert!(line.contains("EQ anchor"), "got: {line}");
    }

    // ─── Recording a finished plan ─────────────────────────────────

    /// A rescue that moved streams records them.
    #[test]
    fn a_rescue_is_recorded() {
        assert_eq!(
            record_action(Some("c1"), Some("c1"), &[38, 71], MovePurpose::Rescue),
            RecordAction::Rescue(vec![38, 71])
        );
    }

    /// A return that moved streams forgets them.
    #[test]
    fn a_return_is_forgotten() {
        assert_eq!(
            record_action(Some("c1"), Some("c1"), &[38], MovePurpose::Return),
            RecordAction::Return(vec![38])
        );
    }

    /// A session that changed while the commands ran invalidates the indices.
    /// They now name whatever took their place, and a later return would move
    /// that.
    #[test]
    fn a_session_change_during_the_move_discards_the_indices() {
        assert_eq!(
            record_action(Some("c1"), Some("c2"), &[38], MovePurpose::Rescue),
            RecordAction::Discard(DiscardReason::SessionChanged)
        );
    }

    /// No cookie at all means the indices cannot be tied to a session. Failing
    /// closed here leaves audio on the bypassed device, which the user can see.
    #[test]
    fn no_cookie_discards_rather_than_guesses() {
        assert_eq!(
            record_action(None, None, &[38], MovePurpose::Rescue),
            RecordAction::Discard(DiscardReason::NoCookie)
        );
    }

    /// A session change is reported as that, not as the missing cookie it also
    /// is. The reason is what the log says, and the actionable one is the
    /// restart.
    #[test]
    fn a_changed_session_is_not_reported_as_a_missing_cookie() {
        assert_eq!(
            record_action(None, Some("c2"), &[38], MovePurpose::Rescue),
            RecordAction::Discard(DiscardReason::SessionChanged)
        );
    }

    /// Nothing moved, nothing to remember — and notably this is not a safety
    /// discard, so the ledger keeps what it already held.
    #[test]
    fn an_empty_move_is_not_a_safety_discard() {
        assert_eq!(
            record_action(Some("c1"), Some("c1"), &[], MovePurpose::Rescue),
            RecordAction::Discard(DiscardReason::NothingMoved)
        );
    }

    // ─── The ledger ─────────────────────────────────────────────────

    /// A new PipeWire session invalidates every index held. Acting on them would
    /// move whichever streams now own those numbers.
    #[test]
    fn a_new_session_discards_what_we_were_holding() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[2628, 4714], Some("first"));
        assert_eq!(ledger.len(), 2);
        assert!(ledger.session_changed(Some("second")));
        assert_eq!(ledger.len(), 0);
    }

    /// The same session keeps it.
    #[test]
    fn the_same_session_keeps_what_we_are_holding() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[2628], Some("first"));
        assert!(!ledger.session_changed(Some("first")));
        assert_eq!(ledger.len(), 1);
    }

    /// A ledger that never got a cookie cannot prove its indices belong to the
    /// current session, so the return path must refuse to act on it.
    #[test]
    fn a_ledger_without_a_cookie_matches_no_session() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[38], None);
        assert!(!ledger.same_session(None));
        assert!(!ledger.same_session(Some("anything")));
    }

    /// Recording under a different cookie than we hold starts from empty rather
    /// than merging two sessions' indices together.
    #[test]
    fn recording_across_sessions_does_not_merge_them() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[1], Some("first"));
        ledger.note_rescued(&[2], Some("second"));
        assert_eq!(ledger.len(), 1);
        assert_eq!(
            ledger.retain_parked(
                &[StreamOnSink {
                    index: 2,
                    sink_index: 74,
                }],
                74
            ),
            vec![2]
        );
    }

    /// The cap keeps a failing move from growing the set forever.
    #[test]
    fn the_ledger_does_not_grow_past_its_cap() {
        let mut ledger = StreamLedger::new(4);
        ledger.note_rescued(&[1, 2, 3, 4, 5, 6, 7, 8], Some("c"));
        assert_eq!(ledger.len(), 4);
    }

    /// The measured case: of the streams rescued, the ones still parked on raw
    /// are the ones to move, and the rest are forgotten because they are already
    /// somewhere they belong.
    #[test]
    fn only_the_still_parked_survive_a_return_pass() {
        let mut ledger = StreamLedger::new(64);
        // 38 is still on raw 74; 168 is on the EPOS 71; 9999 is gone entirely.
        ledger.note_rescued(&[38, 168, 9999], Some("c"));
        assert_eq!(ledger.retain_parked(&listing(), 74), vec![38]);
        assert_eq!(
            ledger.len(),
            1,
            "a stream that came home on its own must be forgotten, or the ledger \
             never empties and no completion is ever reported"
        );
    }

    /// A stream the user moved to their own speakers is forgotten too. Dragging it
    /// back later would overrule a choice made after ours.
    #[test]
    fn a_stream_moved_somewhere_else_is_forgotten_not_returned() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[168], Some("c"));
        assert!(ledger.retain_parked(&listing(), 74).is_empty());
        assert!(ledger.is_empty());
    }

    /// A stream that came home is forgotten, so it is not moved twice.
    #[test]
    fn a_returned_stream_is_forgotten() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[38], Some("c"));
        ledger.note_returned(&[38]);
        assert!(ledger.is_empty());
        assert!(ledger.retain_parked(&listing(), 74).is_empty());
    }

    /// The end-to-end shape: park, the chain dies and recovers, the stream comes
    /// home exactly once, and a second pass does nothing.
    #[test]
    fn a_stream_comes_home_exactly_once() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[38], Some("c"));
        let parked = vec![StreamOnSink {
            index: 38,
            sink_index: 74,
        }];

        let first = ledger.retain_parked(&parked, 74);
        assert_eq!(first, vec![38]);
        ledger.note_returned(&first);

        // now it sits on the anchor
        let home = vec![StreamOnSink {
            index: 38,
            sink_index: 33,
        }];
        assert!(ledger.retain_parked(&home, 74).is_empty());
        assert!(ledger.is_empty());
    }
}
