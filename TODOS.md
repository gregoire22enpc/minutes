# TODOS.md — Minutes

## P2: Calendar Bridge for /minutes prep
**What:** When Google Workspace MCP is available, auto-detect upcoming meetings and pre-populate attendees in /minutes prep. "Is this for your 2pm with Alex?"
**Why:** Makes prep feel magical — it already knows who you're meeting with. Reduces friction to zero.
**Pros:** Leverages existing Google Workspace MCP. Graceful fallback if MCP unavailable.
**Cons:** Adds optional dependency on MCP availability. Needs careful fallback design.
**Context:** Deferred from the interactive skills ecosystem CEO review (2026-03-19). Build after core interactive skills are shipped and validated. The prep skill should work great without calendar — this is a delight layer.
**Effort:** S (human: ~1 day / CC: ~20 min)
**Depends on:** Core interactive skills (prep, debrief, weekly) being built first.

## P2: Proactive Meeting Reminders
**What:** A SessionStart hook that checks if you have a meeting in the next 30-60 minutes and nudges: "You have a call with Alex in 45 minutes. Run /minutes prep?"
**Why:** Makes Minutes feel like a chief of staff that's always aware of your schedule. Proactive > reactive.
**Pros:** Drives skill adoption organically. Creates the "alive" feeling.
**Cons:** Requires calendar bridge (TODO above). Could feel intrusive if poorly calibrated.
**Context:** Deferred from interactive skills ecosystem CEO review (2026-03-19). This is the "chef's kiss" feature that makes the whole system feel proactive.
**Effort:** M (human: ~3 days / CC: ~30 min)
**Depends on:** Calendar bridge + core interactive skills.

## P3: Open Source Interactive Skill Template
**What:** Extract the multi-phase interactive skill pattern into a reusable SKILL-TEMPLATE-INTERACTIVE.md that other Claude Code plugin authors can follow.
**Why:** Positions Minutes as the reference implementation for great Claude Code plugin skills. Community multiplier.
**Pros:** Low effort, high community impact. Documents patterns that would otherwise live only in our heads.
**Cons:** Template may need revision as patterns evolve. Premature extraction risk if patterns aren't battle-tested.
**Context:** Deferred from interactive skills ecosystem CEO review (2026-03-19). Extract after the interactive skills have been used for 2-4 weeks and the patterns are proven.
**Effort:** S (human: ~1 day / CC: ~15 min)
**Depends on:** Interactive skills being battle-tested (2-4 weeks of usage).
