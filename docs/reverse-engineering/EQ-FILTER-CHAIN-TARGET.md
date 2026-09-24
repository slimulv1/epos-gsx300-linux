# Filter-chain instances: why the EQ chain still does not join MAIN

Date: 2026-09-24. Status: **voice chain fixed and verified; EQ chain still broken.**

The `epos-eq` and `epos-voice` instances are both
`libpipewire-module-filter-chain` running in their own software-only
PipeWire daemon. Both were invisible to the main graph, so the EQ, the
voice enhancer and the noise gate all looked enabled in the UI while
doing nothing.

## Fixed: `remote.name` has to be at module args level

With `remote.name = "pipewire-0"` present only inside
`capture.props` / `playback.props`, the module's streams connect back to
their **own** daemon:

```
mod.protocol-native | try_connect() connecting to 'pipewire-epos-eq'
mod.filter-chain    | capture_state_changed(): unconnected
```

The loopback-based sidetone has it at the module level, which is why
sidetone was the only one that ever worked:

```
{ name = libpipewire-module-loopback
  args = {
    remote.name = "pipewire-0"      <-- this line
    ...
```

Moving it to the module args level changes the log to
`connecting to 'pipewire-0'` and both filter-chain instances publish
their nodes into MAIN.

## Fixed: `node.passive` silently disabled the chain

`node.passive = true` on both sides means the chain is never linked, so
it sat at `suspended` even while audio played into it. Replaced with
`node.dont-fallback = true`, which is the actual fail-closed property:
the chain refuses to fall back to a different device when its target
vanishes, but still links to the intended one.

`node.dont-reconnect` is deliberately **not** set — it also gives up when
the intended target merely appears a moment late, which is a normal
startup race rather than a failure.

## Verified working: the voice chain

With those two changes and the daemon-generated conf:

```
epos-voice-capture   Stream/Input/Audio   running
epos-voice-output    Audio/Source         running
```

`epos-voice-output` appears in `pactl list short sources`, and the daemon
now points the default source at it, so the voice enhancer and noise
gate finally affect applications. Fail-closed was verified live: stopping
the voice instance makes the daemon restore the raw device as the
default source within one poll, so the user is never left pointing at a
dead node.

## Still broken: the EQ capture target does not resolve

The EQ chain now reaches MAIN but its capture side fails immediately:

```
mod.filter-chain | capture_state_changed(): error: defined target not found
mod.filter-chain | core_error() res:-2 (No such file or directory)
```

The generated capture side is:

```
capture.props = {
    node.name = "epos-eq-capture"
    target.object = "epos-eq-input.monitor"
    remote.name = "pipewire-0"
    node.dont-fallback = true
}
```

`epos-eq-input` is present in MAIN and is the default sink at the time of
the failure, so the anchor itself is not missing. Two things were tried
and neither resolved it:

- `target.object = "epos-eq-input.monitor"` (current) — unresolvable.
- `target.object = "epos-eq-input"` — also unresolvable.
- Removing `node.dont-reconnect` — no change; the failure is resolution,
  not retry timing.

The sidetone loopback resolves a `target.object` naming an ALSA source
(`alsa_input...mono-fallback`) from the same kind of separate daemon, so
cross-daemon targeting works in principle. The difference is that a
filter-chain capture is a `Stream/Input/Audio`, whose target is normally
a **sink** to record, and the only sink available here is a static
null-sink installed by `40-epos-eq-virtualsink.conf` in a different
config. That mismatch is the prime suspect and has not yet been resolved.

Until it is, the EQ is a no-op. The daemon's output routing already
degrades correctly: with EQ off it routes to the raw hardware sink, and
`epos-eq-input` is only made the default when the EQ is enabled.

## Health-check consequence

`expected_node("eq")` returns the static anchor `epos-eq-input`, which
exists whether or not the chain works. So the post-restart health check
reports EQ healthy even while the chain is missing. It should check
`epos-eq-capture` when the EQ is enabled, the way it checks
`epos-voice-output` for the voice role. Not yet done, because doing it
before the chain works would turn every EQ restart into a false alarm.
