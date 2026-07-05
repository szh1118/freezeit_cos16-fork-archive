# Research: Download Freeze Deferral

## Decision: Use pending-queue retry instead of a new scheduler

**Rationale**: The daemon already represents delayed freezing with `pendingHandleList`. Keeping candidate apps in that queue preserves existing foreground, whitelist, and manual state transitions.

**Alternatives considered**:

- Add a separate download-protected queue: rejected because it duplicates state and risks diverging from existing pending updates.
- Permanently whitelist matching packages: rejected because the user requested deferral only while high-speed downloads are active.

## Decision: Measure receive-byte deltas over a short sample interval

**Rationale**: Download speed requires a rate, not a single counter. The freeze countdown gives a natural one-second cadence for sampling candidates near freeze time.

**Alternatives considered**:

- Sample only at freeze time: rejected because a single counter cannot determine speed.
- Sample every managed app every second: rejected because it would add unnecessary overhead.

## Decision: Read Android 16 UID traffic from `dumpsys netstats`

**Rationale**: The target phone does not expose `/proc/net/xt_qtaguid/stats` or `/proc/uid_stat`. Rooted `dumpsys netstats` exposes `mAppUidStatsMap` with `uid rxBytes rxPackets txBytes txPackets`, which is sufficient for receive-rate decisions.

**Alternatives considered**:

- Parse BPF maps directly: deferred because map ABI and tooling vary across Android releases.
- Use global interface counters: rejected because they cannot distinguish the downloading app from other traffic.

## Decision: Threshold is 5 MiB/s

**Rationale**: The daemon already uses binary units for memory display. Using `5 * 1024 * 1024` bytes per second keeps the threshold explicit and stable.

**Alternatives considered**:

- Decimal 5,000,000 bytes per second: not chosen because existing native code uses binary-style display units.
