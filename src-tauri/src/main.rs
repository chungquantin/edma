// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate base64;

use base64::{engine::general_purpose, Engine};
use serde::Serialize;
use std::process::{Command, Stdio};
use tauri::{Manager, Window};

#[tauri::command]
fn get_shell_path() -> String {
	"/bin/sh".to_string()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Workspace {
	id: i32,
	name: String,
	path: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn execute_command(command: &str) -> Result<String, String> {
	println!("Execute command: {}", command);

	let mut command_builder = Command::new(get_shell_path());

	command_builder.stdin(Stdio::piped()).stderr(Stdio::piped()).stdout(Stdio::piped());

	command_builder.arg("-c").arg(command);
	let child = command_builder.spawn().unwrap();
	let output = child.wait_with_output().expect("Failed to wait on child");
	if output.status.success() {
		let base64_encoded = general_purpose::STANDARD.encode(output.stdout);
		Ok(format!("{:?}", base64_encoded))
	} else {
		let err = String::from_utf8(output.stderr).unwrap();
		Err(err)
	}
}

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_sql::Builder::default().build())
		.setup(|app| {
			let _window: Window = app.get_window("main").unwrap();
			// Prevent initial shaking
			_window.show().unwrap();
			Ok(())
		})
		.on_window_event(|event| match event.event() {
			tauri::WindowEvent::CloseRequested {
				api,
				..
			} => {
				#[cfg(target_os = "macos")]
				{
					event.window().minimize().unwrap();
				}

				#[cfg(not(target_os = "macos"))]
				event.window().close().unwrap();

				api.prevent_close();
			}
			_ => {}
		})
		.invoke_handler(tauri::generate_handler![execute_command, get_shell_path])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
