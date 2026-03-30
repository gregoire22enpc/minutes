---
name: minutes-weekly
description: >
  Review the past week's meetings and surface themes, decision arcs, open
  action items, and stale commitments. Use when the user says "weekly review",
  "what happened this week", "summarize my week", or "Monday brief".
user-invocable: true
allowed-tools:
  - bash
  - read_file
---

# /minutes weekly

Synthesize the past week's meetings into a brief.

## Steps

1. List recent meetings:
```bash
minutes list --since "$(date -d '7 days ago' +%Y-%m-%d 2>/dev/null || date -v-7d +%Y-%m-%d)" --json
```

2. Read each meeting file and extract:
   - **Key decisions** from `decisions:` frontmatter
   - **Open action items** from `action_items:` where `status: open`
   - **People** you met with

3. Synthesize into:
   - **Themes**: recurring topics across meetings
   - **Decision arcs**: decisions that evolved across multiple meetings
   - **Open items**: grouped by assignee, flagging overdue
   - **Stale commitments**: items older than 7 days with no update

4. Present as a concise weekly brief.

## Open action items

```bash
minutes actions                        # All open items
minutes actions --assignee <name>      # Filter by person
```

## Commitment tracking

```bash
minutes commitments                    # All open + overdue
minutes commitments --person <name>    # By person
```
