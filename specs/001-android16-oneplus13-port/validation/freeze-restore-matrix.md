# Freeze/Restore Matrix

## Validation Scope

Validate three non-critical third-party apps selected by the maintainer on the
target OnePlus 13 Android 16 ROM.

## Timing Rules

- Freeze/control must apply within 30 seconds after the app leaves foreground.
- Foreground restore must make the app usable within 5 seconds.
- Record at least 10 restore attempts across the selected apps; at least 9 must
  restore within 5 seconds for SC-004.

## Matrix

| App | Package | UID | Freeze mode | Background time | Controlled within 30s | Restore attempt | Usable within 5s | Result | Evidence |
| --- | --- | ---: | --- | --- | --- | ---: | --- | --- | --- |
| App 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| Any app | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | 10 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |

## Required Diagnostics

- Manager-visible native log around background transition.
- `printFreezerProc` output before and after freeze.
- Evidence that failed/unsupported operations leave the app usable.
- Timing source for background and restore measurements.
