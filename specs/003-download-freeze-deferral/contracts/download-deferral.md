# Contract: Download Freeze Deferral

## Package Matching

Input:

- `package`: Android package name.

Output:

- `true` when the package contains any requested substring:
  - `baidu.netdisk`
  - `quark.clouddrive`
  - `com.google.android.apps.docs`
  - `pikpak`
  - `com.trim.app`
- `false` otherwise.

## Freeze Decision

Input:

- `uid`
- `package`
- current UID receive bytes
- current timestamp
- any previous sample for the same UID

Output:

- `WaitForSample` when the package is a candidate but no usable previous sample exists.
- `Defer` when the package is a candidate and receive rate is greater than 5 MiB/s.
- `Proceed` when the package is not a candidate, stats are unavailable, the sample is invalid, or receive rate is at or below 5 MiB/s.

## Integration Contract

At `Freezer::processPendingApp`:

- Candidate packages with one second remaining should have their receive-byte sample primed.
- Candidate packages at freeze time should call the deferral decision before `handleProcess(appInfo, true)`.
- `WaitForSample` sets the pending countdown to one second.
- `Defer` sets the pending countdown to the retry interval.
- `Proceed` does not change existing freeze behavior.
