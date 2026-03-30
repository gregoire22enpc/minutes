---
name: minutes-prep
description: >
  Interactive meeting preparation — builds a relationship brief and talking
  points before a call. Use when the user says "prep me for my call with",
  "I'm meeting with X", "prepare me for", or "meeting prep".
user-invocable: true
allowed-tools:
  - bash
  - read_file
  - write_file
  - ask_user_question
---

# /minutes prep

Multi-phase interactive meeting preparation. Searches conversation history with a person, synthesizes a relationship brief, and produces talking points.

## Flow

### Phase 1: Who are you meeting with?

Ask the user. If the answer is specific (a name like "Alex"):
```bash
minutes search "<name>" --limit 50
```

If vague ("the team", "everyone"), push back: "Name one person who'll be in the room."

### Phase 2: Relationship brief

Read each matching meeting file. Build:
- **Meeting history**: total meetings, first/most recent, frequency trend
- **Recurring topics**: topics across meetings, trending up/down
- **Open commitments**: items you owe them, items they owe you, overdue flags
- **Decision history**: recent decisions, any volatile ones (changed 2+ times)

Present the brief and move to Phase 3.

### Phase 3: What do you want to accomplish?

Ask: "What's the one thing you'd regret not discussing?"

If vague, push back with evidence from meeting history.

### Phase 4: Save prep file

```bash
mkdir -p ~/.minutes/preps
```

Save to `~/.minutes/preps/YYYY-MM-DD-{firstname}.prep.md` with frontmatter:
```yaml
---
person: {full name}
date: {today ISO}
goal: {stated goal}
meeting_count: {total past meetings}
---
```

Set permissions to 0600.

### Phase 5: Closing

1. Quote something specific the user said during the session
2. Give one concrete pre-meeting action
3. Nudge: "After your call, run `/minutes debrief`"
