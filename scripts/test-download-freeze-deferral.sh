#!/usr/bin/env sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

cat >"$tmp_dir/download_deferral_test.cpp" <<'CPP'
#include "downloadDeferral.hpp"

#include <cassert>
#include <cstdint>
#include <map>

static void assert_action(DownloadDeferralAction actual, DownloadDeferralAction expected) {
    assert(actual == expected);
}

int main() {
    constexpr auto threshold = DownloadFreezeDeferral::DOWNLOAD_THRESHOLD_BYTES_PER_SEC;

    assert(DownloadFreezeDeferral::isCandidatePackage("com.baidu.netdisk"));
    assert(DownloadFreezeDeferral::isCandidatePackage("com.quark.clouddrive"));
    assert(DownloadFreezeDeferral::isCandidatePackage("com.google.android.apps.docs"));
    assert(DownloadFreezeDeferral::isCandidatePackage("com.pikpak.android"));
    assert(DownloadFreezeDeferral::isCandidatePackage("com.trim.app"));
    assert(!DownloadFreezeDeferral::isCandidatePackage("com.reddit.frontpage"));

    DownloadFreezeDeferral deferral;
    auto first = deferral.evaluate(10123, "com.baidu.netdisk", 1000, 100);
    assert_action(first.action, DownloadDeferralAction::WaitForSample);

    auto high_speed = deferral.evaluate(10123, "com.baidu.netdisk", 1000 + threshold + 1, 101);
    assert_action(high_speed.action, DownloadDeferralAction::Defer);
    assert(high_speed.bytesPerSecond == threshold + 1);

    auto slow_first = deferral.evaluate(10124, "com.quark.clouddrive", 2000, 200);
    assert_action(slow_first.action, DownloadDeferralAction::WaitForSample);

    auto at_threshold = deferral.evaluate(10124, "com.quark.clouddrive", 2000 + threshold, 201);
    assert_action(at_threshold.action, DownloadDeferralAction::Proceed);
    assert(at_threshold.bytesPerSecond == threshold);

    auto non_candidate = deferral.evaluate(10125, "com.example.reader", 0, 300);
    assert_action(non_candidate.action, DownloadDeferralAction::Proceed);

    DownloadFreezeDeferral primed;
    primed.primeSample(10126, "com.trim.app", 5000, 400);
    auto primed_high_speed = primed.evaluate(10126, "com.trim.app", 5000 + threshold + 2, 401);
    assert_action(primed_high_speed.action, DownloadDeferralAction::Defer);

    primed.clear(10126);
    auto after_clear = primed.evaluate(10126, "com.trim.app", 1, 500);
    assert_action(after_clear.action, DownloadDeferralAction::WaitForSample);

    const char netstats[] =
        "  mAppUidStatsMap: OK\n"
        "  mStatsMapA: OK\n"
        "  mAppUidStatsMap:\n"
        "    uid rxBytes rxPackets txBytes txPackets\n"
        "    10502 131259371 87762 796372 15107\n"
        "    10140 489026 831 288329 842\n"
        "  mStatsMapA:\n";
    std::map<int, uint64_t> uidRxBytes;
    assert(DownloadFreezeDeferral::parseUidRxBytesMap(netstats, uidRxBytes));
    assert(uidRxBytes.size() == 2);
    assert(uidRxBytes[10502] == 131259371);
    assert(uidRxBytes[10140] == 489026);
    assert(uidRxBytes.count(0) == 0);

    return 0;
}
CPP

cat >"$tmp_dir/netstats.txt" <<'EOF_NETSTATS'
  mAppUidStatsMap: OK
  mStatsMapA: OK
  mAppUidStatsMap:
    uid rxBytes rxPackets txBytes txPackets
    10502 131259371 87762 796372 15107
    10140 489026 831 288329 842
  mStatsMapA:
EOF_NETSTATS
sed -n '/^[[:space:]]*mAppUidStatsMap:[[:space:]]*$/,/^[[:space:]]*mStatsMapA:/p' \
    "$tmp_dir/netstats.txt" >"$tmp_dir/netstats-extracted.txt"
grep -q '10502 131259371' "$tmp_dir/netstats-extracted.txt"
grep -q '10140 489026' "$tmp_dir/netstats-extracted.txt"
if grep -q 'mAppUidStatsMap: OK' "$tmp_dir/netstats-extracted.txt"; then
    echo "netstats extraction included the status line instead of the rx table" >&2
    exit 1
fi

compiler=${CXX:-c++}
"$compiler" -std=c++17 -I "$repo_root/freezeitVS/include" "$tmp_dir/download_deferral_test.cpp" -o "$tmp_dir/download_deferral_test"
"$tmp_dir/download_deferral_test"

grep -q '#include "downloadDeferral.hpp"' "$repo_root/freezeitVS/include/freezer.hpp"
grep -q 'DownloadFreezeDeferral downloadFreezeDeferral' "$repo_root/freezeitVS/include/freezer.hpp"
grep -q 'readUidRxBytes' "$repo_root/freezeitVS/include/freezer.hpp"
grep -q 'shouldDelayFreezeForDownload' "$repo_root/freezeitVS/include/freezer.hpp"
grep -q "UID_RX_BYTES_DUMPSYS_COMMAND" "$repo_root/freezeitVS/include/downloadDeferral.hpp"
grep -q 'parseUidRxBytesMap' "$repo_root/freezeitVS/include/freezer.hpp"
