use std::path;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

enum DiffLine<'a> {
	Equal(&'a str),
	Remove(&'a str),
	Add(&'a str),
}

fn diff_lines<'a>(expected: &'a str, actual: &'a str) -> Vec<DiffLine<'a>> {
	let expected_lines: Vec<_> = expected.lines().collect();
	let actual_lines: Vec<_> = actual.lines().collect();
	let mut dp = vec![vec![0usize; actual_lines.len() + 1]; expected_lines.len() + 1];

	for i in (0..expected_lines.len()).rev() {
		for j in (0..actual_lines.len()).rev() {
			dp[i][j] = if expected_lines[i] == actual_lines[j] {
				dp[i + 1][j + 1] + 1
			}
			else {
				dp[i + 1][j].max(dp[i][j + 1])
			};
		}
	}

	let mut diff = Vec::new();
	let (mut i, mut j) = (0, 0);
	while i < expected_lines.len() && j < actual_lines.len() {
		if expected_lines[i] == actual_lines[j] {
			diff.push(DiffLine::Equal(expected_lines[i]));
			i += 1;
			j += 1;
		}
		else if dp[i + 1][j] >= dp[i][j + 1] {
			diff.push(DiffLine::Remove(expected_lines[i]));
			i += 1;
		}
		else {
			diff.push(DiffLine::Add(actual_lines[j]));
			j += 1;
		}
	}

	while i < expected_lines.len() {
		diff.push(DiffLine::Remove(expected_lines[i]));
		i += 1;
	}

	while j < actual_lines.len() {
		diff.push(DiffLine::Add(actual_lines[j]));
		j += 1;
	}

	diff
}

pub fn render_line_diff(file_path: &path::Path, expected: &str, actual: &str) -> String {
	let diff = diff_lines(expected, actual);
	let mut changed = vec![false; diff.len()];

	for index in 0..diff.len() {
		if !matches!(diff[index], DiffLine::Equal(_)) {
			let start = index.saturating_sub(2);
			let end = (index + 3).min(diff.len());
			for mark in &mut changed[start..end] {
				*mark = true;
			}
		}
	}

	let mut rendered = String::new();
	let mut hidden_equals = false;
	let mut expected_line = 1usize;

	for (index, line) in diff.iter().enumerate() {
		if !changed[index] {
			if matches!(line, DiffLine::Equal(_)) {
				hidden_equals = true;
			}
		}
		else {
			if hidden_equals {
				let file_path = file_path.display();
				rendered.push_str(&format!("{DIM}{file_path}:{expected_line}{RESET}\n"));
				hidden_equals = false;
			}

			match line {
				DiffLine::Equal(text) => {
					rendered.push_str(&format!("{DIM}  {text}{RESET}\n"));
				}
				DiffLine::Remove(text) => {
					rendered.push_str(&format!("{RED}- {text}{RESET}\n"));
				}
				DiffLine::Add(text) => {
					rendered.push_str(&format!("{GREEN}+ {text}{RESET}\n"));
				}
			}
		}

		if matches!(line, DiffLine::Equal(_) | DiffLine::Remove(_)) {
			expected_line += 1;
		}
	}

	rendered
}
