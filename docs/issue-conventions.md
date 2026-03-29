# Issue Conventions

How to write Linear issues so an AI agent can pick them up and implement them without asking for clarification.

## Issue template

Every issue should follow this structure:

```markdown
## Goal

One sentence: what does this change accomplish and why does it matter?

## Background

[Optional] Any context the implementer needs that isn't obvious. Relevant prior decisions, constraints, links to related issues.

## Behaviour / Approach

[Optional for small issues] Describe the expected behaviour or a suggested approach. Don't over-specify — leave room for implementation judgement.

## Acceptance criteria

- [ ] Specific, testable outcome 1
- [ ] Specific, testable outcome 2
- [ ] ...
```

## Rules for writing good issues

**The goal section is mandatory.** One sentence that answers: what is done when this is done?

**Acceptance criteria must be checkable.** Each item should be verifiable without ambiguity. Bad: "it works well". Good: "Running `zeus-skills list` shows all installed skills without error".

**Don't specify the how unless it matters.** Describe what the system should do, not how to implement it. If a specific approach is required (e.g. must use a particular library), say so explicitly and briefly explain why.

**Scope one issue to one deliverable.** If an issue requires decisions across multiple concerns, split it. An agent should be able to complete an issue in a single focused session.

**Put constraints in the issue, not in your head.** If there's a "don't do X" that matters, write it down. Agents don't have context from prior conversations.

## Labels

| Label | When to use |
|---|---|
| `Feature` | New user-facing capability |
| `Bug` | Something is broken or behaving incorrectly |
| `Improvement` | Enhancement to an existing feature |
| `Chore` | Maintenance, tooling, refactor — no user-facing change |

## Workflow states

```
Backlog → Todo → In Progress → In Review → Done
                                         → Cancelled
```

- **Backlog** — exists, not yet prioritised for action
- **Todo** — prioritised, ready to be picked up
- **In Progress** — active work, branch exists (`/pick-issue` sets this)
- **In Review** — PR open, awaiting review (`/submit-work` sets this)
- **Done** — merged and closed
