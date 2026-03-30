---
name: minutes-status
description: >
  Check Minutes recording and processing status. Use when the user asks
  "is anything recording", "what's the status", "am I recording", or
  wants to know the current state of Minutes.
user-invocable: true
allowed-tools:
  - bash
  - read_file
---

# /minutes status

Check the current state of Minutes: idle, recording, or processing.

## Usage

```bash
minutes status
```

Returns JSON with one of three states:
- `{"recording": true, "pid": 12345}` — live capture active
- `{"processing": true}` — capture ended, transcription underway
- `{"recording": false}` — idle

## State files

Minutes tracks state in `~/.minutes/`:
- `recording.pid` — exists while recording is active
- `processing-status.json` — exists during transcription
- `last-result.json` — most recent completed recording

## Quick actions by state

- **Recording** → `minutes stop` to end capture, or `minutes note "..."` to add a timestamped note
- **Processing** → Wait for transcription to finish; check again shortly
- **Idle** → `minutes record` to start, or `minutes list` to browse past meetings
