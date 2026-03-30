---
name: minutes-search
description: >
  Search past meeting transcripts and voice memos for topics, people, decisions,
  or ideas. Use when the user asks "what did we discuss about X", "find that
  meeting where", "what did Alex say", or anything answerable by meeting history.
user-invocable: true
allowed-tools:
  - bash
  - read_file
---

# /minutes search

Find information across all meeting transcripts and voice memos.

## Usage

```bash
minutes search "pricing strategy"                    # Basic search
minutes search "onboarding idea" -t memo             # Voice memos only
minutes search "sprint planning" -t meeting           # Meetings only
minutes search "API redesign" --since 2026-03-01 --limit 5  # Date filter
```

## Flags

| Flag | Description |
|------|-------------|
| `-t, --content-type <meeting\|memo>` | Filter by type |
| `--since <date>` | Only after this date (ISO format) |
| `-l, --limit <n>` | Max results (default: 10) |

## Output

JSON array on stdout. Each result has `title`, `date`, `content_type`, `snippet`, `path`.
Read the full transcript of a match with `cat <path>`.

## Search tips

- Search for what people said, not document titles: `"we should postpone"` not `"launch delay"`
- Search names to find everything someone discussed: `"Alex"`
- Search decisions: `"decided"`, `"agreed"`, `"committed to"`
- Use `--limit` for broad terms to avoid flooding output

## Gotchas

- Search is substring, not fuzzy — typos won't match.
- `--since` needs ISO dates: `2026-03-01`, not "last week".
- Empty results don't mean it wasn't discussed — check `minutes list` for unprocessed recordings.
