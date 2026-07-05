# Data Model: Download Freeze Deferral

## Download Deferral Candidate

Fields:

- `uid`: Android app UID from `ManagedApp`.
- `package`: Managed app package name.
- `label`: Managed app display label for logs.

Validation:

- `package` is eligible when it contains one of the requested substrings.
- Candidate matching is independent from foreground or whitelist state; those existing gates still run separately.

## UID Receive-Byte Sample

Fields:

- `uid`: Android app UID.
- `rxBytes`: Total received bytes reported by system netstats.
- `sampleAt`: Unix timestamp in seconds.

Validation:

- `sampleAt` must increase before a rate can be calculated.
- `rxBytes` must not decrease before a rate can be calculated.

## Deferral Decision

States:

- `Proceed`: Continue normal freeze flow.
- `WaitForSample`: Keep the app pending for the measurement interval because no previous usable sample exists.
- `Defer`: Keep the app pending for a retry interval because receive rate is above threshold.

State transitions:

- No sample -> `WaitForSample`
- Valid sample and rate > threshold -> `Defer`
- Valid sample and rate <= threshold -> `Proceed`
- Invalid or unavailable stats -> `Proceed`
