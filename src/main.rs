use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{env, process::Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Window {
    id: u32,
    title: String,
    app_id: String,
    pid: u32,
    workspace_id: u32,
    is_focused: bool,
    is_floating: bool,
    is_urgent: bool,
    layout: Layout,
    focus_timestamp: Focus_timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Layout {
    pos_in_scrolling_layout: [u32; 2],
    tile_size: [f32; 2],
    window_size: [u32; 2],
    tile_pos_in_workspace_view: Option<u32>,
    window_offset_in_tile: [f32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Focus_timestamp {
    secs: u32,
    nanos: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Workspace {
    id: u32,
    idx: u32,
    name: Option<String>,
    output: String,
    is_urgent: bool,
    is_active: bool,
    is_focused: bool,
    active_window_id: Option<u32>,
}

fn get_windows() -> Result<Vec<Window>> {
    let windows_cmd = Command::new("niri")
        .args(&["msg", "--json", "windows"])
        .output()
        .expect("Failed to execute windows command");

    let windows: Vec<Window> = serde_json::from_slice(&windows_cmd.stdout)?;

    Ok(windows)
}

fn get_emacs_workspace(windows: Vec<Window>) -> u32 {
    for window in windows {
        if window.app_id == "Emacs" {
            return window.workspace_id;
        }
    }

    return 1;
}

fn get_current_workspace() -> Result<u32> {
    let mut current_workspace: u32 = 0;
    let workspace_cmd = Command::new("niri")
        .args(&["msg", "--json", "workspaces"])
        .output()
        .expect("Failed to execute workspaces command");

    let workspaces: Vec<Workspace> = serde_json::from_slice(&workspace_cmd.stdout)?;

    for workspace in workspaces {
        if workspace.is_focused == true {
            current_workspace = workspace.idx;
        }
    }

    Ok(current_workspace)
}

fn execute_emacs_cmd(args: Vec<String>) {
    let _ = Command::new("emacsclient")
        .args(&["-n", "-e", &args[1]])
        .spawn()
        .expect("Failed to execute emacs command");
}

fn switch_workspace(workspace_id: u32) {
    let _ = Command::new("niri")
        .args(&[
            "msg",
            "action",
            "focus-workspace",
            workspace_id.to_string().as_str(),
        ])
        .spawn()
        .expect("Failed to switch workspace");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let windows = get_windows().unwrap();

    let emacs_workspace = get_emacs_workspace(windows);

    let current_workspace = get_current_workspace();

    if current_workspace.unwrap() == emacs_workspace {
        execute_emacs_cmd(args);
    } else {
        switch_workspace(emacs_workspace);
        execute_emacs_cmd(args);
    }
}
