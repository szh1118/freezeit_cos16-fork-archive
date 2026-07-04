# Freezeit Daemon Fixtures

This directory stores protocol, procfs, cgroup, binder, and legacy configuration
fixtures used by daemon contract and integration tests.

Fixtures must be captured from the verified target baseline or from the legacy
module files named in the feature plan. Synthetic fixtures are allowed only when
the test documents the specific edge case that cannot be captured safely from a
device.
