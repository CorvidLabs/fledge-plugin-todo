// fledge-plugin-todo
//
// Scan source files for TODO/FIXME/HACK/XXX comments.
//
// Forked from corvid-agent/fledge-plugin-todo with the following changes:
//  - Filter by source-file extensions (don't scan binaries / lockfiles)
//  - JSON output uses the fledge envelope shape ({schema_version, action, ...})
//  - --all gate (default scans TODO+FIXME; --all adds HACK+XXX)
//  - --limit N to cap results
//  - exit 1 when any TODO/FIXME is found (composable in lanes via `fail-on-todo`)

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ALL_MARKERS: &[&str] = &["TODO", "FIXME", "HACK", "XXX"];
const DEFAULT_MARKERS: &[&str] = &["TODO", "FIXME"];

const SKIP_DIRS: &[&str] = &[
    ".git", "node_modules", "target", ".build", "build", "dist",
    "vendor", "__pycache__", ".venv", "venv", ".next", ".nuxt",
];

const SOURCE_EXTS: &[&str] = &[
    "rs", "py", "js", "jsx", "ts", "tsx", "go", "rb",
    "java", "kt", "swift", "c", "cpp", "h", "hpp",
    "cs", "php", "sh", "bash", "zsh", "fish",
    "lua", "ex", "exs", "erl", "clj", "scala",
];

struct Options {
    dir: String,
    json: bool,
    all: bool,
    limit: Option<usize>,
    fail_on_todo: bool,
}

struct Finding {
    path: PathBuf,
    line: usize,
    marker: String,
    text: String,
}

fn main() {
    let opts = parse_args(env::args().skip(1).collect());
    let markers: &[&str] = if opts.all { ALL_MARKERS } else { DEFAULT_MARKERS };

    let mut findings: Vec<Finding> = Vec::new();
    walk(Path::new(&opts.dir), markers, &mut findings);

    if let Some(n) = opts.limit {
        findings.truncate(n);
    }

    if opts.json {
        print_json(&opts, markers, &findings);
    } else {
        print_table(markers, &findings);
    }

    if opts.fail_on_todo && !findings.is_empty() {
        std::process::exit(1);
    }
}

fn parse_args(args: Vec<String>) -> Options {
    let mut opts = Options {
        dir: ".".to_string(),
        json: false,
        all: false,
        limit: None,
        fail_on_todo: false,
    };

    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        match a.as_str() {
            "--json" => opts.json = true,
            "--all" => opts.all = true,
            "--fail-on-todo" => opts.fail_on_todo = true,
            "--limit" | "-l" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                    opts.limit = Some(v);
                    i += 1;
                }
            }
            _ => {
                if !a.starts_with('-') {
                    opts.dir = a.clone();
                }
            }
        }
        i += 1;
    }

    opts
}

fn walk(dir: &Path, markers: &[&str], findings: &mut Vec<Finding>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            // Skip standard build/cache directories and any dotted directory.
            if !SKIP_DIRS.contains(&name.as_ref()) && !name.starts_with('.') {
                walk(&path, markers, findings);
            }
        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if SOURCE_EXTS.contains(&ext) {
                    scan_file(&path, markers, findings);
                }
            }
        }
    }
}

fn scan_file(path: &Path, markers: &[&str], findings: &mut Vec<Finding>) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };

    for (i, line) in content.lines().enumerate() {
        for marker in markers {
            if let Some(pos) = line.find(marker) {
                // Avoid matching inside identifiers like `MY_TODO_LIST`.
                let before_ok = pos == 0
                    || !line.as_bytes()[pos - 1].is_ascii_alphanumeric()
                        && line.as_bytes()[pos - 1] != b'_';
                let after = pos + marker.len();
                let after_ok = after >= line.len()
                    || !line.as_bytes()[after].is_ascii_alphanumeric()
                        && line.as_bytes()[after] != b'_';
                if !before_ok || !after_ok {
                    continue;
                }
                let rest = line[after..].trim_start_matches([':', ' ', '(', '-']);
                let text = rest.trim();
                let truncated = if text.len() > 200 { &text[..200] } else { text };
                findings.push(Finding {
                    path: path.to_path_buf(),
                    line: i + 1,
                    marker: marker.to_string(),
                    text: truncated.to_string(),
                });
                break;
            }
        }
    }
}

fn print_table(markers: &[&str], findings: &[Finding]) {
    if findings.is_empty() {
        println!("No TODO/FIXME comments found.");
        return;
    }

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for f in findings {
        *counts.entry(f.marker.as_str()).or_default() += 1;
    }

    println!("Found {} item(s):", findings.len());
    for marker in markers {
        if let Some(&count) = counts.get(marker) {
            println!("  {}: {}", marker, count);
        }
    }
    println!();

    for f in findings {
        println!(
            "  {}:{} [{}] {}",
            f.path.display(),
            f.line,
            f.marker,
            f.text
        );
    }
}

fn print_json(opts: &Options, markers: &[&str], findings: &[Finding]) {
    let mut s = String::new();
    s.push_str(r#"{"schema_version":1,"action":"todo","root":""#);
    s.push_str(&escape(&opts.dir));
    s.push_str(r#"","tags_searched":["#);
    for (i, m) in markers.iter().enumerate() {
        if i > 0 { s.push(','); }
        s.push('"');
        s.push_str(m);
        s.push('"');
    }
    s.push_str(r#"],"count":"#);
    s.push_str(&findings.len().to_string());
    s.push_str(r#","matches":["#);
    for (i, f) in findings.iter().enumerate() {
        if i > 0 { s.push(','); }
        s.push_str(r#"{"file":""#);
        s.push_str(&escape(&f.path.display().to_string().replace('\\', "/")));
        s.push_str(r#"","line":"#);
        s.push_str(&f.line.to_string());
        s.push_str(r#","tag":""#);
        s.push_str(&f.marker);
        s.push_str(r#"","text":""#);
        s.push_str(&escape(&f.text));
        s.push_str(r#""}"#);
    }
    s.push_str("]}");
    println!("{}", s);
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}
