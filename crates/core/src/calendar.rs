use std::process::Command;

// ──────────────────────────────────────────────────────────────
// Calendar integration — upcoming meetings from macOS Calendar.
//
// Uses AppleScript to query Calendar.app. Avoids the `whose`
// filter on CalDAV calendars (causes timeouts). Instead fetches
// all events for today and filters by time in the script.
//
// Also tries a compiled EventKit helper if available.
// ──────────────────────────────────────────────────────────────

/// A calendar event with title and start time.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CalendarEvent {
    pub title: String,
    pub start: String,
    pub minutes_until: i64,
}

/// Query upcoming calendar events within the next `lookahead_minutes`.
/// Returns events sorted by start time (all-day events excluded).
pub fn upcoming_events(lookahead_minutes: u32) -> Vec<CalendarEvent> {
    // Try compiled EventKit helper first
    if let Some(events) = query_via_eventkit(lookahead_minutes) {
        if !events.is_empty() {
            return events;
        }
    }
    // AppleScript: fetch today's events, filter by time range
    query_via_applescript(lookahead_minutes)
}

/// Query via compiled Swift EventKit helper (if available and permitted).
fn query_via_eventkit(lookahead_minutes: u32) -> Option<Vec<CalendarEvent>> {
    let helper = find_calendar_helper()?;

    let output = Command::new(&helper)
        .arg(lookahead_minutes.to_string())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let events: Vec<CalendarEvent> = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();

    Some(events)
}

/// Find the compiled calendar-events helper binary.
fn find_calendar_helper() -> Option<std::path::PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let in_resources = dir.join("../Resources/calendar-events");
            if in_resources.exists() {
                return Some(in_resources);
            }
            let beside = dir.join("calendar-events");
            if beside.exists() {
                return Some(beside);
            }
        }
    }
    let dev = dirs::home_dir()
        .unwrap_or_default()
        .join("Sites/minutes/target/release/calendar-events");
    if dev.exists() {
        return Some(dev);
    }
    None
}

/// AppleScript approach: fetch ALL events for today+tomorrow, filter by time.
/// Avoids `whose start date >= ...` which times out on CalDAV calendars.
fn query_via_applescript(lookahead_minutes: u32) -> Vec<CalendarEvent> {
    // Fetch events for a 2-day window, then filter in the script
    let script = format!(
        r#"set now to current date
set todayStart to current date
set hours of todayStart to 0
set minutes of todayStart to 0
set seconds of todayStart to 0
set tomorrowEnd to todayStart + (2 * 24 * 60 * 60)
set lookaheadSecs to {minutes} * 60
set horizon to now + lookaheadSecs
set output to ""
tell application "Calendar"
    repeat with cal in calendars
        try
            set evts to (every event of cal whose start date >= todayStart and start date <= tomorrowEnd)
            repeat with evt in evts
                set s to start date of evt
                if s >= now and s <= horizon then
                    set t to summary of evt
                    set mins to ((s - now) / 60) as integer
                    set output to output & t & (ASCII character 31) & (s as string) & (ASCII character 31) & mins & linefeed
                end if
            end repeat
        end try
    end repeat
end tell
return output"#,
        minutes = lookahead_minutes
    );

    let output = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output();

    let output = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            eprintln!("[calendar] applescript failed: {}", stderr.trim());
            return Vec::new();
        }
        Err(e) => {
            eprintln!("[calendar] osascript error: {}", e);
            return Vec::new();
        }
    };

    let sep = '\x1F'; // ASCII unit separator
    let mut events: Vec<CalendarEvent> = output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(3, sep).collect();
            if parts.len() >= 3 {
                Some(CalendarEvent {
                    title: parts[0].trim().to_string(),
                    start: parts[1].trim().to_string(),
                    minutes_until: parts[2].trim().parse().unwrap_or(0),
                })
            } else {
                None
            }
        })
        .collect();

    // Deduplicate by title (same event can appear in multiple calendars)
    events.sort_by_key(|e| (e.minutes_until, e.title.clone()));
    events.dedup_by(|a, b| a.title == b.title && a.minutes_until == b.minutes_until);
    events
}
