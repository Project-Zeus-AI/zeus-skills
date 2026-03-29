# Agentic Development Workflow

How to work on a Linear issue using Claude Code and the Project Zeus AI harness. From picking up an issue to merging a PR, without leaving your editor.

## Prerequisites

- **Claude Code** installed and running (`claude`)
- **Linear MCP server** configured in Claude Code settings (`linear-server`)
- **GitHub CLI** installed and authenticated (`gh auth status`)
- **Harness skills** installed: `zeus-skills install Project-Zeus-AI/zeus-skills-library`

Verify skills are available by typing `/pick-issue` — you should see the skill trigger.

---

## 1. Start a session

Open Claude Code in your project directory:

```bash
claude
```

The `CLAUDE.md` in your project root gives the agent full context automatically. You don't need to explain the project.

---

## 2. Pick an issue

Invoke the skill:

```
/pick-issue
```

The agent will:
1. Ask for a Linear issue identifier if you haven't provided one (e.g. `PRO-12`)
2. Fetch the full issue — title, description, acceptance criteria
3. Create a git branch using the issue's `gitBranchName`
4. Set the issue to **In Progress** in Linear
5. Brief you on the acceptance criteria and ask how to proceed

You can also specify the issue inline:

```
/pick-issue PRO-12
```

---

## 3. Work on the issue

Describe what you want done, or tell the agent to proceed. The agent has the full issue context — you don't need to repeat it.

**Tips:**

- For complex issues, ask the agent to break it down before coding: `/plan-issue` *(M2)*
- Keep sessions focused on one issue at a time
- If you need a break, use `/update-issue` to log progress before closing

### Posting a mid-session update

```
/update-issue
```

Posts a concise progress comment to the Linear issue based on what's been done. Useful before stepping away or handing off.

---

## 4. Submit work

When the implementation is complete:

```
/submit-work
```

The agent will:
1. Commit any uncommitted changes with a descriptive message
2. Push the branch
3. Create a GitHub PR titled `<identifier>: <title>`
4. Post a comment on the Linear issue with the PR link and a summary
5. Move the issue to **In Review**

---

## 5. Review and merge

The PR is now open. Review it on GitHub in the usual way.

Once approved:
1. Merge the PR on GitHub
2. The Linear issue is left as **In Review** until you manually move it to **Done** (or a GitHub Action does it — see M4)

If changes are requested:
1. Return to Claude Code in the same project directory
2. Run `/pick-issue <identifier>` — the agent will detect the open PR and load review comments as context
3. Address the feedback, then run `/submit-work` again to push a new commit to the same PR

---

## 6. Troubleshooting

**`/pick-issue` doesn't respond**
Skills may not be installed. Run `zeus-skills list` to check. If missing: `zeus-skills install Project-Zeus-AI/zeus-skills-library`.

**Branch already exists**
The agent will check it out rather than creating a new one. Work continues normally.

**`/submit-work` fails on push**
The branch may have diverged. Don't force push — investigate with `git status` and `git log`. Resolve manually, then re-run `/submit-work`.

**Linear status not updating**
Confirm the `linear-server` MCP is connected (`/mcp` in Claude Code). Re-authenticate if needed.

**Agent lost context mid-session**
Run `/pick-issue <identifier>` again. It will re-fetch the issue and reload context, including any open PR and comments.
