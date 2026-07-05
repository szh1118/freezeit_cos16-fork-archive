#pragma once

#include <cctype>
#include <cstdint>
#include <ctime>
#include <limits>
#include <map>
#include <string_view>

enum class DownloadDeferralAction {
    Proceed,
    WaitForSample,
    Defer,
};

struct DownloadDeferralDecision {
    DownloadDeferralAction action = DownloadDeferralAction::Proceed;
    uint64_t bytesPerSecond = 0;
};

class DownloadFreezeDeferral {
public:
    static constexpr uint64_t DOWNLOAD_THRESHOLD_BYTES_PER_SEC = 5ULL * 1024ULL * 1024ULL;
    static constexpr int MEASUREMENT_INTERVAL_SECONDS = 1;
    static constexpr int RETRY_DELAY_SECONDS = 30;
    static constexpr const char* UID_RX_BYTES_DUMPSYS_COMMAND =
        "/system/bin/dumpsys netstats | /system/bin/sed -n "
        "'/^[[:space:]]*mAppUidStatsMap:[[:space:]]*$/,/^[[:space:]]*mStatsMapA:/p'";

    static bool isCandidatePackage(const std::string_view package) {
        constexpr std::string_view patterns[] = {
            "baidu.netdisk",
            "quark.clouddrive",
            "com.google.android.apps.docs",
            "pikpak",
            "com.trim.app",
        };

        for (const auto pattern : patterns)
            if (containsCaseInsensitive(package, pattern))
                return true;
        return false;
    }

    static bool parseUidRxBytesMap(const std::string_view netstats,
        std::map<int, uint64_t>& uidRxBytes) {
        uidRxBytes.clear();

        bool inAppUidStatsMap = false;
        size_t offset = 0;
        while (offset <= netstats.length()) {
            const auto next = netstats.find('\n', offset);
            auto line = next == std::string_view::npos ?
                netstats.substr(offset) : netstats.substr(offset, next - offset);
            line = trimAscii(line);

            if (line == "mAppUidStatsMap:") {
                inAppUidStatsMap = true;
            }
            else if (inAppUidStatsMap && line == "mStatsMapA:") {
                break;
            }
            else if (inAppUidStatsMap) {
                auto fields = line;
                uint64_t uid = 0;
                uint64_t rxBytes = 0;
                if (parseUnsigned(fields, uid) && parseUnsigned(fields, rxBytes)
                    && uid <= static_cast<uint64_t>(std::numeric_limits<int>::max())) {
                    uidRxBytes[static_cast<int>(uid)] = rxBytes;
                }
            }

            if (next == std::string_view::npos)
                break;
            offset = next + 1;
        }

        return !uidRxBytes.empty();
    }

    void primeSample(const int uid, const std::string_view package,
        const uint64_t rxBytes, const time_t sampleAt) {
        if (!isCandidatePackage(package))
            return;
        samples[uid] = { rxBytes, sampleAt };
    }

    DownloadDeferralDecision evaluate(const int uid, const std::string_view package,
        const uint64_t rxBytes, const time_t sampleAt) {
        DownloadDeferralDecision decision;

        if (!isCandidatePackage(package))
            return decision;

        const auto it = samples.find(uid);
        if (it == samples.end()) {
            samples[uid] = { rxBytes, sampleAt };
            decision.action = DownloadDeferralAction::WaitForSample;
            return decision;
        }

        const auto previous = it->second;
        samples[uid] = { rxBytes, sampleAt };

        if (sampleAt <= previous.sampleAt || rxBytes < previous.rxBytes)
            return decision;

        const auto elapsedSeconds = static_cast<uint64_t>(sampleAt - previous.sampleAt);
        decision.bytesPerSecond = (rxBytes - previous.rxBytes) / elapsedSeconds;
        if (decision.bytesPerSecond > DOWNLOAD_THRESHOLD_BYTES_PER_SEC)
            decision.action = DownloadDeferralAction::Defer;
        return decision;
    }

    void clear(const int uid) {
        samples.erase(uid);
    }

private:
    struct Sample {
        uint64_t rxBytes = 0;
        time_t sampleAt = 0;
    };

    std::map<int, Sample> samples;

    static std::string_view trimAscii(std::string_view text) {
        while (!text.empty() && std::isspace(static_cast<unsigned char>(text.front())))
            text.remove_prefix(1);
        while (!text.empty() && std::isspace(static_cast<unsigned char>(text.back())))
            text.remove_suffix(1);
        return text;
    }

    static bool parseUnsigned(std::string_view& text, uint64_t& value) {
        text = trimAscii(text);
        if (text.empty() || !std::isdigit(static_cast<unsigned char>(text.front())))
            return false;

        value = 0;
        size_t consumed = 0;
        while (consumed < text.length()
            && std::isdigit(static_cast<unsigned char>(text[consumed]))) {
            value = value * 10ULL + static_cast<uint64_t>(text[consumed] - '0');
            consumed++;
        }
        text.remove_prefix(consumed);
        return true;
    }

    static bool containsCaseInsensitive(const std::string_view text,
        const std::string_view pattern) {
        if (pattern.empty())
            return true;
        if (text.length() < pattern.length())
            return false;

        for (size_t i = 0; i + pattern.length() <= text.length(); i++) {
            bool matched = true;
            for (size_t j = 0; j < pattern.length(); j++) {
                const auto left = static_cast<unsigned char>(text[i + j]);
                const auto right = static_cast<unsigned char>(pattern[j]);
                if (std::tolower(left) != std::tolower(right)) {
                    matched = false;
                    break;
                }
            }
            if (matched)
                return true;
        }
        return false;
    }
};
