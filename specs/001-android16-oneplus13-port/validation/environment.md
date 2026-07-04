# Environment Evidence

Captured: 2026-07-03

## Gradle

- Command: `bash freezeitApp/gradlew --version`
- Result: `PASS`
- Version: Gradle 8.4
- JVM: 26.0.1 on Linux amd64
- Note: Gradle emits restricted native-access warnings from the wrapper runtime;
  the version command exits successfully.

## Android NDK

- Path: `/home/admin/Android/Sdk/ndk/28.2.13676358`
- Command:
  `/home/admin/Android/Sdk/ndk/28.2.13676358/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang++ --version`
- Result: `PASS`
- Compiler: Android clang 19.0.1
- Target: `aarch64-unknown-linux-android35`

## adevtool

- Path: `/home/admin/.local/bin/adevtool`
- Command: `/home/admin/.local/bin/adevtool --help`
- Result: `PASS`
- Version: `adevtool/1.0.0 linux-x64 node-v26.4.0`

## Target ROM Archive

- Path: `/home/admin/code/Rom/oneplus13.zip`
- Command: `ls -lh /home/admin/code/Rom/oneplus13.zip`
- Result: `PASS`
- Size: `8090.2M`

## MIO-KITCHEN-SOURCE

- Path: `/home/admin/code/MIO-KITCHEN-SOURCE`
- Command: `ls -la /home/admin/code/MIO-KITCHEN-SOURCE`
- Result: `PASS`
- Evidence: checkout contains `.venv/`, `requirements.txt`, `build.py`,
  `tool.py`, `src/`, and `bin/`.
- Open issue for T007: local smoke run previously failed with
  `No module named 'google'`; dependency repair is tracked separately in
  `mio-setup.md`.
