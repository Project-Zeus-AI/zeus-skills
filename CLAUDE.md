# zeus-skills

CLI tool for managing Claude Code skills on the client machine. Installs, removes, creates, and updates skills in `~/.claude/skills/`. Supports single-skill repos and monorepos (subdirs each containing `SKILL.md`).

## Linear

- **Team key:** PRO
- **Project:** Agentic Engineering Harness
- **MCP server:** linear-server (available in this session)

Use `/pick-issue` to start work on a Linear issue. Use `/submit-work` when done. Use `/update-issue` to post a progress comment mid-session.

## Tech stack

- **Language:** Rust (edition 2024)
- **Key dependencies:** clap 4 (derive), anyhow, dirs 5

## Build & run

```bash
# Build
cargo build

# Run (dev)
cargo run -- <command>
# e.g. cargo run -- list
# e.g. cargo run -- install Project-Zeus-AI/zeus-skills-library

# Run tests
cargo test

# Install globally
cargo install --path .
```

## Project structure

```
zeus-skills/
├── src/
│   └── main.rs          # all logic in one file for now
├── templates/
│   └── CLAUDE.md        # generic CLAUDE.md template for new harness projects
├── Cargo.toml
├── Cargo.lock
└── CLAUDE.md            # this file
```

## Skills storage

- Installed skills: `~/.claude/skills/<skill-name>/`
- Monorepo cache: `~/.zeus-skills/repos/<repo-name>/`
- Each skill directory must contain a `SKILL.md` with YAML frontmatter (`name`, `description`)

## Branching & commits

- Branch format: `<github-username>/<issue-id>-<short-slug>` (e.g. `em0ney/pro-10-claudemd-template`)
- Use the `gitBranchName` from the Linear issue exactly
- Commit messages: imperative mood
- Do not commit directly to `main`

## Skills installed

```bash
zeus-skills install Project-Zeus-AI/zeus-skills-library
```

| Skill | Invoke | When to use |
|---|---|---|
| pick-issue | `/pick-issue` | Starting work on a Linear issue |
| submit-work | `/submit-work` | Shipping completed work for review |
| update-issue | `/update-issue` | Posting a mid-session progress update |

## Do's and don'ts

- **Do** keep all logic in `src/main.rs` until complexity clearly warrants splitting
- **Do** test CLI commands manually with `cargo run --` before submitting
- **Don't** add dependencies without a clear reason — the binary should stay small
- **Don't** use `unwrap()` — use `anyhow` context propagation throughout
