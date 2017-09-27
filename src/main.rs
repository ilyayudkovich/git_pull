use std::process::{Command, Output};
use std::str;
use std::thread;
use std::time::Duration;

static GIT_DIR: &'static str = "/home/iyudkovich/git/";

fn git_command(args: &[&str], dir: &str) -> Output {
	let output = Command::new("git")
		        .args(args)
		        .current_dir(dir)
			    .output()
			    .expect("Failed to generate git command");
	return output;
}

fn git_pull(dir: &str) {
	println!("about to pull for  {:?}", dir);
	git_command(&["pull", "--rebase"], dir);
}

fn get_dirs() -> String {
	let output = Command::new("ls")
			.current_dir(GIT_DIR)
			.output()
			.expect("failed to execute ls command");
	return String::from_utf8(output.stdout).expect("Not from_utf8");
}

fn is_git(dir: &str) -> bool {
	let output = git_command(&["rev-parse", "--is-inside-work-tree"], dir);
	let mut is_git: String = String::from_utf8(output.stdout)
										.expect("wasn't utf8");
	is_git.pop();
	match is_git.as_ref() {
		"true" => true,
		_ => false,
	}
}

fn main() {
	let dirs: String = get_dirs();
	let vec_dirs: Vec<String> = dirs.split("\n").map(
		|s| s.to_string()).collect();
	for dir in vec_dirs {
		let mut owned_string = String::from(GIT_DIR);
		owned_string.push_str(&dir);
		if is_git(&owned_string) {
			git_pull(&owned_string);
		}
	}
}