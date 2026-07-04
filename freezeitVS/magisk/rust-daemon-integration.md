# Rust Daemon Integration Notes

The modern daemon build produces a `freezeit` binary for the Magisk module root.
During the transition, the existing C++ binary remains the behavioral reference
for manager protocol compatibility, legacy policy migration, and install script
behavior.

Integration points:

- `customize.sh` owns architecture selection, executable permissions, and manager
  APK installation.
- `service.sh` owns boot-completed and first-unlock waiting before daemon start.
- The Rust daemon must preserve the module root data files `appcfg.txt`,
  `applabel.txt`, `settings.db`, `boot.log`, and manager-visible logs until a
  migration task records a compatible replacement.
- The release package must contain exactly one executable daemon named
  `freezeit` after installation.
