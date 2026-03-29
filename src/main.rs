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
    /// Install a skill from a GitHub repo (e.g. owner/repo or https://github.com/owner/repo)
    Install { source: String },
    /// Remove an installed skill by name
    Remove { name: String },
    /// Scaffold a new skill
    Create { name: String },
    /// Update all installed skills (git pull in each skill directory)
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

fn install_skill(source: &str) -> Result<()> {
    let url = normalise_github_url(source);
    let name = repo_name_from_url(&url)
        .context("Could not determine skill name from source. Use owner/repo format.")?;
    let dest = skills_dir()?.join(&name);

    if dest.exists() {
        bail!("Skill '{name}' is already installed. Use `zeus-skills update` to update it.");
    }

    println!("Installing '{name}' from {url}...");
    let status = Command::new("git")
        .args(["clone", "--depth=1", &url, dest.to_str().unwrap()])
        .status()
        .context("Failed to run git. Is git installed?")?;

    if !status.success() {
        bail!("git clone failed for {url}");
    }

    println!("Installed '{name}'.");
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
    let dir = skills_dir()?;
    let skills: Vec<_> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter(|e| e.path().join(".git").exists())
        .collect();

    if skills.is_empty() {
        println!("No git-managed skills to update.");
        return Ok(());
    }

    let mut updated = 0;
    let mut failed = 0;
    for entry in &skills {
        let name = entry.file_name().into_string().unwrap_or_default();
        print!("Updating '{name}'... ");
        let status = Command::new("git")
            .args(["-C", entry.path().to_str().unwrap(), "pull", "--ff-only"])
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

    println!("\n{updated} updated, {failed} failed.");
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
