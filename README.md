# zeus-skills

CLI tool for managing Claude Code skills on the client machine.

## Install

```bash
cargo install --git https://github.com/Project-Zeus-AI/zeus-skills
```

## Commands

```
zeus-skills list                        # list all installed skills
zeus-skills install <owner/repo>        # install a skill from GitHub
zeus-skills remove <name>               # remove an installed skill
zeus-skills create <name>               # scaffold a new skill
zeus-skills update                      # update all git-managed skills
```

## Usage

### Install a skill from GitHub

```bash
zeus-skills install Project-Zeus-AI/my-skill
# or with a full URL
zeus-skills install https://github.com/Project-Zeus-AI/my-skill
```

Skills are cloned into `~/.claude/skills/<repo-name>/`.

### Create a new skill

```bash
zeus-skills create my-skill
```

Scaffolds `~/.claude/skills/my-skill/SKILL.md` with the required frontmatter template.

### Update all skills

```bash
zeus-skills update
```

Runs `git pull --ff-only` in every skill directory that is a git repo.

## Skill format

Each skill is a directory in `~/.claude/skills/` containing a `SKILL.md` file:

```markdown
---
name: my-skill
description: One-line description shown in `zeus-skills list`
metadata:
  trigger: When to use this skill
  author: Your name
---

# My Skill

Skill prompt content here.
```

Additional files (templates, references, scripts) can live alongside `SKILL.md` in the same directory.
