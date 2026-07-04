# MIO Setup Evidence

Captured: 2026-07-03

## Dependency Check

- Command:
  `/home/admin/code/MIO-KITCHEN-SOURCE/.venv/bin/python -c 'import google; print(google.__path__)'`
- Result: `PASS`
- Evidence:
  `_NamespacePath(['/home/admin/code/MIO-KITCHEN-SOURCE/.venv/lib/python3.14/site-packages/google'])`

The previously observed `No module named 'google'` issue is not present in the
current MIO virtual environment.

## MIO Startup Check

- Command:
  `/home/admin/code/MIO-KITCHEN-SOURCE/.venv/bin/python /home/admin/code/MIO-KITCHEN-SOURCE/tool.py --help`
- Result: `DEGRADED`
- Current blocker:
  `_tkinter.TclError: no display name and no $DISPLAY environment variable`

`tool.py` starts the Tk UI even for `--help`, so it cannot be used directly from
this headless shell. For non-interactive evidence tasks, use MIO source modules
or fallback archive tools when they can extract the required ROM baseline facts.

## Requirements File

- Path: `/home/admin/code/MIO-KITCHEN-SOURCE/requirements.txt`
- Notable dependency: `protobuf>7`, which supplies the `google.protobuf`
  namespace used by Android payload metadata tooling.

## Follow-Up

Use a graphical session, an X virtual framebuffer, or a non-GUI MIO entrypoint
before relying on the full Tk workflow for framework image unpacking. The setup
issue is documented rather than blocking the baseline archive inspection.
