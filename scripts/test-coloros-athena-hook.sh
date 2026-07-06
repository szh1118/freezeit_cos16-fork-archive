#!/usr/bin/env sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

require_text() {
  file=$1
  text=$2
  message=$3
  if ! grep -Fq "$text" "$file"; then
    echo "$message" >&2
    echo "missing text: $text" >&2
    echo "file: $file" >&2
    exit 1
  fi
}

require_line() {
  file=$1
  text=$2
  message=$3
  if ! grep -Fxq "$text" "$file"; then
    echo "$message" >&2
    echo "missing line: $text" >&2
    echo "file: $file" >&2
    exit 1
  fi
}

enum_java="$repo_root/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/Enum.java"
entry_java="$repo_root/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/FreezeitHookEntry.java"
modern_java="$repo_root/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/ModernHook.java"
athena_java="$repo_root/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/app/OplusAthena.java"
arrays_xml="$repo_root/freezeitApp/app/src/main/res/values/arrays.xml"
scope_list="$repo_root/freezeitApp/app/src/main/resources/META-INF/xposed/scope.list"

require_text "$enum_java" 'oplusAthena = "com.oplus.athena"' "Athena package constant is missing"
require_text "$arrays_xml" '<item>com.oplus.athena</item>' "Legacy Xposed recommended scope is missing Athena"
require_line "$scope_list" 'com.oplus.athena' "Modern Xposed scope list is missing Athena"

require_text "$entry_java" 'case Enum.Package.oplusAthena:' "Legacy package dispatch is missing Athena"
require_text "$entry_java" 'OplusAthena.Hook(classLoader);' "Athena hook is not dispatched"
require_text "$modern_java" 'Enum.Package.oplusAthena.equals(packageName)' "Modern hook allowlist is missing Athena"

require_text "$athena_java" 'OplusForceStopStrategy' "ForceStopStrategy hook target is missing"
require_text "$athena_java" 'OplusKillPidStrategy' "KillPidStrategy hook target is missing"
require_text "$athena_java" 'OplusKillUidStrategy' "KillUidStrategy hook target is missing"
require_text "$athena_java" 'OplusForceStopOrKillStrategy' "ForceStopOrKillStrategy hook target is missing"
require_text "$athena_java" 'Enum.Method.oplusForceStop' "Athena force-stop utility hook is missing"
require_text "$athena_java" 'Enum.Method.oplusForceStopWithFlag' "Athena force-stop flag utility hook is missing"
require_text "$athena_java" 'Enum.Method.oplusKillSimple' "Athena simple kill utility hook is missing"
require_text "$athena_java" 'Enum.Method.oplusKill' "Athena kill utility hook is missing"
require_text "$athena_java" 'Enum.Method.oplusClearActionKill' "Athena clear action kill wrapper hook is missing"
require_text "$athena_java" 'onPowerProtectPolicyChange' "GuardElf policy diagnostic hook is missing"
require_text "$athena_java" 'setGuardElfSwitch' "GuardElf switch diagnostic hook is missing"
require_text "$athena_java" 'param.setResult(new ArrayList<>())' "External clear strategy hook must return an empty list"
require_text "$athena_java" 'param.setResult(false)' "Kill utility hook must return false"

require_text "$entry_java" 'case Enum.Package.powerkeeper:' "Existing MIUI PowerKeeper dispatch was removed"
require_text "$entry_java" 'case Enum.Package.android:' "Existing Android system dispatch was removed"
