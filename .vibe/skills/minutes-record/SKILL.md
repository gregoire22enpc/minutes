---
name: minutes-record
description: >
  Start or stop recording a meeting, call, or voice memo. Use when the user
  says "record", "start recording", "capture this meeting", "stop recording",
  or wants to transcribe live audio.
user-invocable: true
allowed-tools:
  - bash
  - read_file
---

# /minutes record

Record audio from the microphone, transcribe locally with whisper.cpp, and save as searchable markdown.

## Start recording

```bash
minutes record
# With a title:
minutes record --title "Weekly standup with Alex"
```

The process captures audio from the default input device. It runs in the foreground until stopped.

## Stop recording

```bash
minutes stop
```

This signals the recording process, which then:
1. Stops audio capture
2. Transcribes via whisper.cpp (local, nothing leaves the machine)
3. Saves to `~/meetings/YYYY-MM-DD-title.md`
4. Prints the output path and word count as JSON

## Check status

```bash
minutes status
```

Returns JSON: `{"recording": true, "pid": 12345}` or `{"recording": false}`

## First-time setup

If the user hasn't set up minutes yet:
```bash
minutes setup --model small   # 466MB, good accuracy
minutes setup --model tiny    # 75MB, faster but less accurate
```

## Common issues

- **"model not found"** → Run `minutes setup --model small`.
- **"already recording"** → Run `minutes stop` first, or `minutes status` to check.
- **No audio / empty transcript** → Check input device in System Settings > Sound.
- **For Zoom/Meet/Teams audio** → Needs BlackHole for system audio capture.
- **Long meetings (>2h)** → Transcription scales with duration; `small` model takes ~3-5 min for 2h on Apple Silicon.
