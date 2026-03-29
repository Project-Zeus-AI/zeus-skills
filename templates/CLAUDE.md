# [PROJECT NAME]

[One sentence describing what this project does and why it exists.]

## Linear

- **Team key:** [e.g. PRO]
- **Project:** [Project name as it appears in Linear]
- **MCP server:** linear-server (available in this session)

Use `/pick-issue` to start work on a Linear issue. Use `/submit-work` when done. Use `/update-issue` to post a progress comment mid-session.

## Tech stack

- **Language:** [e.g. Rust / TypeScript / Python]
- **Framework:** [e.g. None / Next.js / FastAPI]
- **Key dependencies:** [list main ones]

## Build & run

```bash
# Install dependencies
[command]

# Build
[command]

# Run
[command]

# Run tests
[command]
```

## Project structure

```
[paste a brief tree here — top 2 levels is enough]
```

## Branching & commits

- Branch format: `<github-username>/<issue-id>-<short-slug>` (e.g. `em0ney/pro-5-skill-management-cli`)
- Use the `gitBranchName` from the Linear issue exactly
- Commit messages: imperative mood, describe the "what", not the "how"
- Do not commit directly to `main`

## Skills installed

Install all skills with:
```bash
zeus-skills install Project-Zeus-AI/zeus-skills-library
```

| Skill | Invoke | When to use |
|---|---|---|
| pick-issue | `/pick-issue` | Starting work on a Linear issue |
| submit-work | `/submit-work` | Shipping completed work for review |
| update-issue | `/update-issue` | Posting a mid-session progress update |

## Do's and don'ts

- **Do** write tests for any non-trivial logic
- **Do** check the acceptance criteria in the Linear issue before submitting
- **Don't** add features beyond what the issue asks for
- **Don't** modify unrelated files in the same commit

## [Project-specific context]

[Add anything else an agent needs to know to work in this project without asking questions: auth quirks, env vars, known gotchas, etc.]
