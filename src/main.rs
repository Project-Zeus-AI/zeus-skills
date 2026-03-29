use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "zeus-skills", about = "Manage Claude Code skills on the client machine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all installed skills
    List,
    /// Install skills from a GitHub repo (owner/repo or https://github.com/owner/repo).
    /// Monorepos (subdirectories each containing SKILL.md) are fully supported.
    Install { source: String },
    /// Remove an installed skill by name
    Remove { name: String },
    /// Scaffold a new skill
    Create { name: String },
    /// Update all installed skills
    Update,
}

fn skills_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let dir = home.join(".claude").join("skills");
    if !dir.exists() {
        fs::create_dir_all(&dir).context("Could not create ~/.claude/skills")?;
    }
    Ok(dir)
}

/// Cache directory for monorepo clones: ~/.zeus-skills/repos/<name>
fn repos_cache_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let dir = home.join(".zeus-skills").join("repos");
    if !dir.exists() {
        fs::create_dir_all(&dir).context("Could not create ~/.zeus-skills/repos")?;
    }
    Ok(dir)
}

fn list_skills() -> Result<()> {
    let dir = skills_dir()?;
    let mut skills: Vec<String> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.'))
        .collect();

    if skills.is_empty() {
        println!("No skills installed.");
        return Ok(());
    }

    skills.sort();
    println!("Installed skills ({}):", skills.len());
    for skill in &skills {
        let skill_md = dir.join(skill).join("SKILL.md");
        let description = if skill_md.exists() {
            read_skill_description(&skill_md).unwrap_or_default()
        } else {
            String::new()
        };
        if description.is_empty() {
            println!("  {skill}");
        } else {
            println!("  {skill} — {description}");
        }
    }
    Ok(())
}

fn read_skill_description(skill_md: &PathBuf) -> Option<String> {
    let content = fs::read_to_string(skill_md).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("description:") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

/// Returns true if the directory is a monorepo: contains subdirectories with SKILL.md
/// but no SKILL.md at the root itself.
fn is_monorepo(dir: &PathBuf) -> bool {
    if dir.join("SKILL.md").exists() {
        return false;
    }
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .any(|e| e.path().join("SKILL.md").exists())
        })
        .unwrap_or(false)
}

fn install_skill(source: &str) -> Result<()> {
    let url = normalise_github_url(source);
    let repo_name = repo_name_from_url(&url)
        .context("Could not determine repo name from source. Use owner/repo format.")?;

    // Clone to a temp location first to inspect
    let tmp = tempdir()?;
    println!("Cloning {url}...");
    let status = Command::new("git")
        .args(["clone", "--depth=1", &url, tmp.to_str().unwrap()])
        .status()
        .context("Failed to run git. Is git installed?")?;

    if !status.success() {
        bail!("git clone failed for {url}");
    }

    if is_monorepo(&tmp) {
        install_from_monorepo(&tmp, &repo_name, &url)
    } else {
        install_single_skill(&tmp, &repo_name)
    }
}

fn install_single_skill(cloned: &PathBuf, name: &str) -> Result<()> {
    let dest = skills_dir()?.join(name);
    if dest.exists() {
        bail!("Skill '{name}' is already installed. Use `zeus-skills update` to update it.");
    }
    fs::rename(cloned, &dest)
        .or_else(|_| copy_dir_all(cloned, &dest))
        .with_context(|| format!("Could not install skill '{name}'"))?;
    println!("Installed '{name}'.");
    Ok(())
}

fn install_from_monorepo(cloned: &PathBuf, repo_name: &str, url: &str) -> Result<()> {
    // Cache the full clone for future updates
    let cache = repos_cache_dir()?.join(repo_name);
    if cache.exists() {
        bail!("Skills from '{repo_name}' are already installed. Use `zeus-skills update` to update them.");
    }
    fs::rename(cloned, &cache)
        .or_else(|_| copy_dir_all(cloned, &cache))
        .with_context(|| format!("Could not cache repo '{repo_name}'"))?;

    // Write the remote URL so we can pull later
    fs::write(cache.join(".zeus-source"), url)?;

    let skills_dir = skills_dir()?;
    let mut installed = Vec::new();

    for entry in fs::read_dir(&cache)?.filter_map(|e| e.ok()) {
        let skill_dir = entry.path();
        if !skill_dir.is_dir() {
            continue;
        }
        let name = entry.file_name().into_string().unwrap_or_default();
        if name.starts_with('.') || !skill_dir.join("SKILL.md").exists() {
            continue;
        }
        let dest = skills_dir.join(&name);
        if dest.exists() {
            println!("  Skipping '{name}' (already installed).");
            continue;
        }
        copy_dir_all(&skill_dir, &dest)
            .with_context(|| format!("Could not copy skill '{name}'"))?;

        installed.push(name);
    }

    if installed.is_empty() {
        println!("No skills found in '{repo_name}'.");
    } else {
        println!("Installed {} skill(s) from '{repo_name}':", installed.len());
        for name in &installed {
            println!("  {name}");
        }
    }
    Ok(())
}

fn remove_skill(name: &str) -> Result<()> {
    let dest = skills_dir()?.join(name);
    if !dest.exists() {
        bail!("Skill '{name}' is not installed.");
    }
    fs::remove_dir_all(&dest).with_context(|| format!("Could not remove '{name}'"))?;
    println!("Removed '{name}'.");
    Ok(())
}

fn create_skill(name: &str) -> Result<()> {
    let dest = skills_dir()?.join(name);
    if dest.exists() {
        bail!("A skill named '{name}' already exists.");
    }
    fs::create_dir_all(&dest)?;

    let skill_md = format!(
        r#"---
name: {name}
description: TODO: describe what this skill does and when to use it
metadata:
  trigger: TODO: describe what prompts or contexts should trigger this skill
  author: TODO
---

# {name}

TODO: write the skill prompt here.
"#
    );

    fs::write(dest.join("SKILL.md"), skill_md)?;
    println!("Created skill '{name}' at ~/.claude/skills/{name}/SKILL.md");
    Ok(())
}

fn update_skills() -> Result<()> {
    let mut updated = 0;
    let mut failed = 0;
    let skills_dir = skills_dir()?;

    // Update monorepo caches and re-sync skill copies
    let cache_dir = repos_cache_dir()?;
    if let Ok(entries) = fs::read_dir(&cache_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_dir() || !path.join(".git").exists() {
                continue;
            }
            let name = entry.file_name().into_string().unwrap_or_default();
            print!("Updating '{name}'... ");
            let status = Command::new("git")
                .args(["-C", path.to_str().unwrap(), "pull", "--ff-only"])
                .status();
            match status {
                Ok(s) if s.success() => {
                    // Re-sync skill subdirectories into ~/.claude/skills/
                    if let Ok(skill_entries) = fs::read_dir(&path) {
                        for skill_entry in skill_entries.filter_map(|e| e.ok()) {
                            let skill_path = skill_entry.path();
                            let skill_name = skill_entry.file_name().into_string().unwrap_or_default();
                            if skill_name.starts_with('.') || !skill_path.is_dir() || !skill_path.join("SKILL.md").exists() {
                                continue;
                            }
                            let dest = skills_dir.join(&skill_name);
                            let _ = fs::remove_dir_all(&dest);
                            let _ = copy_dir_all(&skill_path, &dest);
                        }
                    }
                    println!("done.");
                    updated += 1;
                }
                _ => {
                    println!("failed.");
                    failed += 1;
                }
            }
        }
    }

    // Also update any directly-cloned skills
    for entry in fs::read_dir(&skills_dir)?.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = entry.file_name().into_string().unwrap_or_default();
        if name.starts_with('.') || !path.is_dir() || !path.join(".git").exists() {
            continue;
        }
        print!("Updating '{name}'... ");
        let status = Command::new("git")
            .args(["-C", path.to_str().unwrap(), "pull", "--ff-only"])
            .status();
        match status {
            Ok(s) if s.success() => {
                println!("done.");
                updated += 1;
            }
            _ => {
                println!("failed.");
                failed += 1;
            }
        }
    }

    if updated == 0 && failed == 0 {
        println!("No git-managed skills to update.");
    } else {
        println!("\n{updated} updated, {failed} failed.");
    }
    Ok(())
}

fn normalise_github_url(source: &str) -> String {
    if source.starts_with("http://") || source.starts_with("https://") || source.starts_with("git@")
    {
        return source.to_string();
    }
    format!("https://github.com/{source}")
}

fn repo_name_from_url(url: &str) -> Option<String> {
    url.trim_end_matches('/')
        .split('/')
        .last()
        .map(|s| s.trim_end_matches(".git").to_string())
        .filter(|s| !s.is_empty())
}

fn tempdir() -> Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("zeus-skills-{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)?.filter_map(|e| e.ok()) {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => list_skills(),
        Commands::Install { source } => install_skill(&source),
        Commands::Remove { name } => remove_skill(&name),
        Commands::Create { name } => create_skill(&name),
        Commands::Update => update_skills(),
    }
}
