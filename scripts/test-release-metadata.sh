#!/usr/bin/env sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
version="3.2.8SelfUse"
version_code="302008"
release_zip="freezeit_oneplus13_android16_selfuse_v${version}_${version_code}.zip"
repo_url="https://github.com/szh1118/freezeit_cos16"
raw_url="https://raw.githubusercontent.com/szh1118/freezeit_cos16/main"
mirror_raw_url="https://mirror.ghproxy.com/${raw_url}"
author="JARK006 / @szh1118"

require_text() {
    file="$1"
    text="$2"
    if ! grep -F "$text" "$repo_root/$file" >/dev/null; then
        echo "missing expected text in $file: $text" >&2
        exit 1
    fi
}

require_text freezeitApp/app/build.gradle "versionCode ${version_code}"
require_text freezeitApp/app/build.gradle "versionName \"${version}\""

require_text freezeitVS/magisk/module.prop "version=${version}"
require_text freezeitVS/magisk/module.prop "versionCode=${version_code}"
require_text freezeitVS/magisk/module.prop "author=${author}"
require_text freezeitVS/src/main.cpp "By ${author}"

require_text freezeitRelease/update.json "\"version\": \"${version}\""
require_text freezeitRelease/update.json "\"versionCode\": ${version_code}"
require_text freezeitRelease/update.json "${mirror_raw_url}/freezeitRelease/${release_zip}"
require_text freezeitRelease/update.json "${mirror_raw_url}/freezeitRelease/changelog.txt"

require_text freezeitApp/app/src/main/res/values/strings.xml "<string name=\"github_link\" translatable=\"false\">https://github.com/szh1118</string>"
require_text freezeitApp/app/src/main/res/values/strings.xml "<string name=\"github_project_link\" translatable=\"false\">${repo_url}</string>"
require_text freezeitApp/app/src/main/res/values/strings.xml "<string name=\"github_app_link\" translatable=\"false\">${repo_url}/tree/main/freezeitApp</string>"
require_text freezeitApp/app/src/main/res/values/strings.xml "<string name=\"developer\">Developer ${author}</string>"
require_text freezeitApp/app/src/main/res/values/strings.xml "https://github.com/szh1118/freezeit_cos16/issues"

require_text freezeitApp/app/src/main/res/values-zh/strings.xml "<string name=\"developer\">开发者 ${author}</string>"
require_text freezeitApp/app/src/main/res/values-zh/strings.xml "${mirror_raw_url}/freezeitRelease/update.json"
require_text freezeitApp/app/src/main/res/values-zh/strings.xml "${mirror_raw_url}/freezeitRelease/changelogFull.txt"

require_text README.md "Module version: \`${version}\` / versionCode \`${version_code}\`"
require_text README.md "freezeitRelease/${release_zip}"
require_text README.md "## ${version} Changes"

require_text freezeitRelease/README.md "freezeitRelease/${release_zip}"
require_text freezeitRelease/README.md "\`${version}\` is a background runtime-control build"
require_text freezeitRelease/changelog.txt "### v${version} validation notes 2026-07-08"
require_text freezeitRelease/changelogFull.txt "### v${version} 更新日志 2026-07-08"
require_text freezeitVS/changelog.txt "### v${version} validation notes 2026-07-08"
