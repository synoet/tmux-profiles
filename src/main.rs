mod cli;
use clap::Parser;
use cli::Cli;
use core::panic;
use serde::{Deserialize, Serialize};
use std::process::exit;
use toml::from_str;
use anyhow::{Result, Context};

#[derive(Serialize, Deserialize, Debug)]
struct TmuxPane {
    location: String,
    command: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TmuxWindow {
    panes: Vec<TmuxPane>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TmuxSession {
    name: String,
    group: Option<String>,
    windows: Vec<TmuxWindow>,
    select: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(rename = "session")]
    sessions: Vec<TmuxSession>,
}

fn get_current_sessions() -> Vec<String> {
    let result: String = match std::process::Command::new( "sh").arg("-c")
    .arg("tmux list-sessions -F \"#{session_name}\"").output() {
        Ok(output) => {
            String::from_utf8(output.stdout).unwrap_or("".to_owned())
        },
        Err(_e) => {
            return vec![];
        },
    };

    let sessions: Vec<String> = result.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_owned()).collect();

    sessions
}

fn kill_session(name: &str) -> Result<()>{
    std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("tmux kill-session -t {}", name))
        .spawn()
        .context("failed to kill session")?;
    Ok(())
}

fn create(session: &TmuxSession, force: bool) -> Result<()> {
    let current_sessions = get_current_sessions();
    match (current_sessions.contains(&session.name), force) {
        (true, true) => kill_session(&session.name)?,
        (true, false) => {
            eprintln!("ERROR: session {} already exists, use --force to recreate", &session.name);
            exit(1);
        },
        _ => (),
    }

    let mut command: String = format!("tmux new -s {}", &session.name);
    if session.windows.len() == 0 {
        eprintln!("ERROR: no windows defined in session {}", &session.name);
        exit(1);
    }

    let initial_window = &session.windows[0];
    command.push_str(&format!(
        " -n {} -c {} \\;",
        &initial_window.name, &initial_window.panes[0].location
    ));

    if let Some(command_) = &initial_window.panes[0].command {
        command.push_str(&format!(
            " send-keys -t {} \" {}\" C-m \\;",
            &initial_window.name, &command_
        ))
    }

    for window in &session.windows[1..] {
        command.push_str(&format!(
            " new-window -n {} -c {} \\;",
            &window.name, &window.panes[0].location
        ));
        if let Some(command_) = &window.panes[0].command {
            command.push_str(&format!(
                " send-keys -t {} \" {}\" C-m \\;",
                &window.name, &command_
            ))
        }
        dbg!(&window);
        for pane in &window.panes[1..] {
            command.push_str(&format!(" split-window -c {} \\;", &pane.location));
            dbg!(&pane);
            if let Some(command_) = &pane.command {
                command.push_str(&format!(
                    " send-keys -t {} \" {}\" C-m \\;",
                    &window.name, &command_
                ))
            }
        }
    }

    dbg!(&command);

    std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .spawn()?;

    Ok(())
}

fn launch (config: &Config, name: String,  force: bool) -> Result<()> {
    let session: &TmuxSession = match config.sessions.iter().find(|s| s.name == name) {
        Some(s) => s,
        None => {
            eprintln!("ERROR: no profile named {}", &name);
            exit(1);
        }
    };

    create(session, force)?;

    Ok(())
}

fn launch_group(config: &Config, name: String, force: bool) -> Result<()>{
    let sessions: Vec<&TmuxSession> = config.sessions
        .iter()
        .filter(|s| {
            if let Some(group) = &s.group {
                return group == &name;
            }
            false
        })
        .collect();

    for session in sessions {
        create(session, force)?;
    }

    Ok(())
}

fn list(config: &Config) -> Result<()> {
    println!("available profiles:");
    for session in config.sessions.iter() {
        print!("  - {}", session.name);
        if let Some(group) = &session.group {
            print!(" (group: {})", group);
        }
        println!();
    }

    Ok(())
}

fn main() {
    let matches = Cli::parse();

    let home = match std::env::var("HOME") {
        Ok(home) => home,
        _ => panic!("no $HOME env var set"),
    };

    let path = format!("{}/{}", &home, "tmux-profiles.toml");

    let config: Config = match std::fs::read_to_string(&path) {
        Ok(content) => match from_str::<Config>(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("ERROR: failed to parse config {}", e);
                exit(1)
            }
        },
        _ => {
            eprintln!("ERROR: no config found at {}", &path);
            exit(1);
        }
    };


    if let Err(err) = match matches.command { 
        cli::Commands::Launch { name, force } => launch(&config, name, force.unwrap_or(false)),
        cli::Commands::Group { name, force } => launch_group(&config, name, force.unwrap_or(false)),
        cli::Commands::List => list(&config),
    } {
        eprintln!("{}", err);
        exit(1);
    }
}
