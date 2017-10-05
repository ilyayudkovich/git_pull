use std::process::{Command, Output};
use std::str;
use std::io;
use std::{thread, time};

fn git_pull(dir: &str) {
	println!("about to pull for  {:?}", dir);
    let pull = Command::new("git")
    					.args(&["pull", "--rebase"])
    					.current_dir(dir)
    					.spawn()
    					.expect("failed to pull repo");
}

fn get_dirs(dir: &str) -> String {
	println!("Getting dirs from {:?}", dir);
	let output = Command::new("ls")
			.current_dir(dir)
			.output()
			.expect("failed to execute ls command");
	return String::from_utf8(output.stdout).expect("Not from_utf8");
}

fn is_git(dir: &str) -> bool {
	println!("Checking to see if git {:?}", dir);
	let mut output = Command::new("git")
						   .args(&["rev-parse", "--is-inside-work-tree"])
						   .current_dir(dir)
						   .output()
						   .expect("failed to check if git repo");

	let mut is_git: String = String::from_utf8(output.stdout)
										.expect("wasn't utf8");
	is_git.pop();
	match is_git.as_ref() {
		"true" => true,
		_ => false,
	}
}

fn main() {
	println!("Enter absolute path to main git directory:");
	// prompt for the git directory
	let mut root = String::new();
	io::stdin().read_line(&mut root)
			   .expect("Failed to read line")
			   .to_string();

	root.pop();

	println!("{:?}", &root);
	let vec_dirs: Vec<String> =
		get_dirs(&root).split("\n").map(
			|s| s.to_string()).collect();

	for dir in vec_dirs {
		let mut git_dir = root.to_owned();
		git_dir.push_str(&dir);
		if is_git(git_dir.as_str()) {
			git_pull(git_dir.as_str());
			thread::sleep_ms(400); // for clarity
		}
		git_dir.clear();
	}
}