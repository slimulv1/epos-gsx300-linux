# EQ architecture: options, and why the current one stays

Date: 2026-09-24. Priority: **stability and compatibility**, over elegance.

## What the EQ has to satisfy

1. Apps must reach EQ'd audio, and the choice must survive app restarts.
2. When the DSP path is unavailable, audio must keep working. Losing the EQ
   is acceptable; losing the audio is not.
3. Changing a band must not interrupt unrelated system audio.
4. A failure must be detected and repaired without the user doing anything.
5. It must not require restarting the main PipeWire daemon.

## The topology in use

```
app ──> epos-eq-input (static null-sink, MAIN, from 40-epos-eq-virtualsink.conf)
          └── monitor ──> epos-eq-capture ──filter.graph──> epos-eq-output
                                                                     │
                                                          EPOS ALSA hardware sink
```

Three separate moving parts, which is the main reason this has needed so
much care:

- the null-sink is a **static MAIN node**, so it exists even while the DSP is
  down. That is deliberate: apps stay pinned and go quiet rather than being
  re-routed to the laptop speakers.
- the filter-chain runs in a **separate, software-only PipeWire daemon**
  (`pipewire-epos@eq`), so a bad generated conf cannot take down A2+ audio.
- the daemon **moves the default sink** to the anchor while the EQ is on, and
  moves it back when the chain is not usable.

## Alternatives considered

### A. Keep the current topology — recommended

Every failure mode has been exercised on this machine:

| Event | Behaviour |
| --- | --- |
| `pipewire-epos@eq` killed | Playback moves to raw hardware within one 5 s poll, the instance is restarted, playback returns to the EQ about 9 s later. Already-playing streams are moved explicitly, so they are audible rather than pinned to a dead anchor. |
| Main `pipewire.service` restarted | Anchor is re-created, chain is absent ~5 s, playback falls back to raw, then returns to the EQ by itself. |
| Daemon killed with `kill -9` | EQ audio is unaffected: the DSP instances are separate systemd units and the anchor is a static MAIN node. Only the default-sink steering depends on the daemon. |
| Headset unplugged | The chain's playback target disappears, the nodes go, and the route falls back. |
| Conf cannot be written | The route stays on raw rather than pointing at a sink nobody drains. |

This is the only option tested against every one of those events, and it
passes them.

### B. Put the filter-chain in the MAIN graph

Simpler topology, and it removes the cross-daemon target resolution entirely
— that is the mechanism behind the `defined target not found` class of
problem, and the reason the capture side must not set
`node.dont-fallback`.

Rejected: `pipewire.conf.d` is only parsed at daemon startup, so **every EQ
band change would require restarting the main PipeWire daemon**, interrupting
every application on the system. That fails requirement 3 outright.

### C. Let the session manager do the linking

Instead of `target.object` on the capture, use `node.autoconnect` and a
WirePlumber policy to link `epos-eq-capture` to the anchor's monitor.
PipeWire's own guidance favours this for robustness, because a name-based
target has to be re-resolved while a session-manager link is managed
continuously.

Not adopted, for two reasons. It adds a WirePlumber policy as a new moving
part, and this project has already measured a WirePlumber rule moving zero
streams for a related routing task — the same machinery is not reliable here
by default. It is worth an experiment, but it is a replacement for
something that currently works and is self-healing, not a fix for a known
fault.

### D. `module-parametric-equalizer`

PipeWire ships a module purpose-built for EQ that expands to a biquad
filter-chain internally, rather than the hand-generated graph in
`generate_eq_instance_conf`. Fewer moving parts in our own code and
upstream-correct band semantics.

Worth evaluating if the EQ ever needs to be more than a fixed biquad chain.
It is a rewrite of the graph generation, with no defect currently forcing
it, so it is not justified now.

### E. Put DSP in ALSA

LADSPA in the ALSA path, ahead of the USB device. No PipeWire involvement at
all, so no graph or session-manager risk — but it is per-device ALSA
configuration, it does not follow PipeWire's stream routing, and it cannot
express the "EQ only for some apps" behaviour the anchor currently gives
for free. Rejected on compatibility grounds.

## Invariants the current design depends on

These are load-bearing. Each was found by measurement, and each has a test
or a comment pinning it.

1. `remote.name = "pipewire-0"` must be at the **filter-chain module args
   level**. With it only inside `capture.props`/`playback.props`, the
   module connects back to its own daemon and publishes nothing.
2. `node.passive` must not be set on either side — it prevents the chain
   from ever being linked.
3. `node.dont-fallback` must **not** be on the EQ capture side. It is
   correct and kept on the playback side, but on the capture it makes the
   module abandon the target lookup and fail. This single property was the
   whole reason the EQ appeared dead.
4. The EQ chain is healthy only when **both** `epos-eq-capture` and
   `epos-eq-output` are published.
5. A probe that times out is `Unknown`, never `Absent`. Only a successful
   probe that does not see the nodes counts against the chain.
6. `pactl set-default-sink` only affects new streams, so a fallback must
   also move already-playing streams off the anchor.

## Remaining known gaps

Not defects that lose audio today, but not solved either:

- **No loaded-conf identity.** `write_instance_conf` compares on-disk bytes
  only, so it cannot detect an instance running an *older* conf whose file
  already matches. Mitigated by restarting the DSP instances unconditionally
  at daemon startup, which makes the running state a function of the config
  rather than of luck. The proper fix is a hash the instance records and the
  daemon compares.
- **Shutdown can drop a pending restart.** A `Quit` or SIGTERM between
  writing a conf and the debounce worker draining loses the restart; the next
  start would see unchanged bytes. The same startup convergence covers it.
- **External `pactl` routing fights the daemon.** The watchdog re-asserts the
  default sink every 5 s when the EQ is on, so a deliberate manual change to
  the default sink is reverted. Intentional, but worth knowing.
- **All-flat bands degrade to passthrough** while `eq.enabled` stays true, so
  the status reports the EQ as active when no filtering happens. Honest fix
  would be an effective-band count in the status response.
