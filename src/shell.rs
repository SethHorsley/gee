use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn update_shell_config(gee_dir: &PathBuf) -> Result<()> {
    let shells = vec!["bash", "zsh", "fish"];
    let home_dir = dirs::home_dir().context("Failed to get home directory")?;

    for shell in shells {
        let rc_file = match shell {
            "bash" => home_dir.join(".bashrc"),
            "zsh" => home_dir.join(".zshrc"),
            "fish" => home_dir.join(".config/fish/config.fish"),
            _ => continue,
        };

        if rc_file.exists() {
            let content = fs::read_to_string(&rc_file)?;
            let gee_dir_str = gee_dir.to_string_lossy();
            if !content.contains(gee_dir_str.as_ref()) {
                let append_content = match shell {
                    "fish" => format!("\nset -gx PATH $PATH {}\n", gee_dir.display()),
                    _ => format!("\nexport PATH=\"$PATH:{}\"\n", gee_dir.display()),
                };
                fs::write(&rc_file, content + &append_content)?;
                println!("Updated {} configuration", shell);
            }
        }
    }

    Ok(())
}

