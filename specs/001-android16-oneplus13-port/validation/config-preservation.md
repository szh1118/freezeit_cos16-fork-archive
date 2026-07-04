# Upgrade Config Preservation

Captured: 2026-07-03

## Source

- File: `freezeitVS/magisk/customize.sh`

## Existing Installed Paths

| Config file | Existing path | New module destination | Status |
| --- | --- | --- | --- |
| `appcfg.txt` | `/data/adb/modules/freezeit/appcfg.txt` | `$MODPATH/appcfg.txt` | `PASS` |
| `applabel.txt` | `/data/adb/modules/freezeit/applabel.txt` | `$MODPATH/applabel.txt` | `PASS` |
| `settings.db` | `/data/adb/modules/freezeit/settings.db` | `$MODPATH/settings.db` | `PASS` |

## Evidence

`customize.sh` defines:

- `ORG_appcfg="/data/adb/modules/freezeit/appcfg.txt"`
- `ORG_applabel="/data/adb/modules/freezeit/applabel.txt"`
- `ORG_settings="/data/adb/modules/freezeit/settings.db"`

It then iterates over those paths and copies each existing file into `$MODPATH`
with `cp -f`.

## Result

The current upgrade script preserves the required config files when an older
installed module provides them. No migration change is required for the current
contract.
