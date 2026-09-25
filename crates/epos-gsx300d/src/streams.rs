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

/// One stream's sink, as resolved by the caller from a listing.
#[allow(dead_code)] // The return path is the next commit; nothing calls this yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamOnSink {
    pub index: u32,
    pub sink_index: u32,
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
#[allow(dead_code)] // The return path is the next commit; nothing calls this yet.
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
#[allow(dead_code)] // The return path is the next commit; nothing calls this yet.
pub fn session_cookie(pactl_info: &str) -> Option<&str> {
    pactl_info.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("Cookie:")?;
        let value = rest.trim();
        (!value.is_empty()).then_some(value)
    })
}

/// What the daemon remembers about streams it moved, and whether it may still act.
#[derive(Debug, Default)]
#[allow(dead_code)] // The return path is the next commit; nothing calls this yet.
pub struct StreamLedger {
    rescued: BTreeSet<u32>,
    /// The PipeWire session those indices belong to.
    cookie: Option<String>,
    /// The largest ledger worth keeping. A set that only grows is a leak, and the
    /// graph has tens of streams, not thousands.
    cap: usize,
}

#[allow(dead_code)] // The return path is the next commit; nothing calls this yet.
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

    /// The streams to bring back, given what is actually on the graph now.
    pub fn plan_return(&self, present: &[StreamOnSink], raw_index: u32) -> Vec<u32> {
        streams_to_return(&self.rescued, present, raw_index)
    }

    /// Drop indices whose streams are gone, so the set cannot grow forever.
    pub fn prune(&mut self, present: &[StreamOnSink]) {
        let live: BTreeSet<u32> = present.iter().map(|s| s.index).collect();
        self.rescued.retain(|i| live.contains(i));
    }

    /// Forget a stream the daemon has successfully brought back.
    pub fn note_returned(&mut self, indices: &[u32]) {
        for i in indices {
            self.rescued.remove(i);
        }
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

    /// Recording under a different cookie than we hold starts from empty rather
    /// than merging two sessions' indices together.
    #[test]
    fn recording_across_sessions_does_not_merge_them() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[1], Some("first"));
        ledger.note_rescued(&[2], Some("second"));
        assert_eq!(ledger.len(), 1);
        assert!(
            ledger
                .plan_return(
                    &[StreamOnSink {
                        index: 2,
                        sink_index: 74,
                    }],
                    74
                )
                .contains(&2)
        );
    }

    /// The cap keeps a failing move from growing the set forever.
    #[test]
    fn the_ledger_does_not_grow_past_its_cap() {
        let mut ledger = StreamLedger::new(4);
        ledger.note_rescued(&[1, 2, 3, 4, 5, 6, 7, 8], Some("c"));
        assert_eq!(ledger.len(), 4);
    }

    /// Streams that have gone are dropped, so the set tracks the graph.
    #[test]
    fn pruning_drops_streams_that_are_gone() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[38, 9999], Some("c"));
        ledger.prune(&listing());
        assert_eq!(ledger.len(), 1);
        assert_eq!(ledger.plan_return(&listing(), 74), vec![38]);
    }

    /// A stream that came home is forgotten, so it is not moved twice.
    #[test]
    fn a_returned_stream_is_forgotten() {
        let mut ledger = StreamLedger::new(64);
        ledger.note_rescued(&[38], Some("c"));
        ledger.note_returned(&[38]);
        assert!(ledger.is_empty());
        assert!(ledger.plan_return(&listing(), 74).is_empty());
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

        let first = ledger.plan_return(&parked, 74);
        assert_eq!(first, vec![38]);
        ledger.note_returned(&first);

        // now it sits on the anchor
        let home = vec![StreamOnSink {
            index: 38,
            sink_index: 33,
        }];
        assert!(ledger.plan_return(&home, 74).is_empty());
    }
}
