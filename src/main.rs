use std::fs;
use std::io::Write;
use anyhow::{Context, Result};
use clap::Parser;

// NOTE: uncomment levels as needed
// mod level1;
// mod level2;
// mod level3;
// mod level4;
// mod level5;
// mod level6;
// mod level7;

const FORCE_WRITE: bool = false;

const INPUT_DIR: &str = "src/input";
const OUTPUT_DIR: &str = "src/output";

#[derive(Parser)]
struct Cli {
    level: usize,
    sub_level: Option<usize>,
}

fn read_lines(path: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read file at path: {}", path))?;
    let lines = content.lines().map(str::to_string).collect();
    Ok(lines)
}

fn build_path(level: usize, sub_level: &str, is_output: bool) -> String {
    let (dir, file_type) = if is_output { (OUTPUT_DIR, "out") } else { (INPUT_DIR, "in") };
    format!("{}/level{}/level{}_{}.{}", dir, level, level, sub_level, file_type)
}

fn get_level_input_path(args: &Cli) -> Result<String> {
    let sub_level = match args.sub_level {
        Some(sub_level) => sub_level.to_string(),
        None => "example".to_string()
    };
    Ok(build_path(args.level, &sub_level, false))
}

fn compare_output(output: Vec<String>, level: usize) -> Result<()> {
    let expected_output_path = format!("{}/level{}/level{}_example.out", INPUT_DIR, level, level);
    let expected_output = read_lines(&expected_output_path)?;

    if FORCE_WRITE || output == expected_output {
        if output == expected_output && !FORCE_WRITE {
            println!("Solution matches expected output!");
        } else if FORCE_WRITE {
            println!(
                "-------- Forced Output --------\n{}\n",
                output.join("\n")
            );
        }

        generate_output_files(level)?;
    } else {
        println!(
            "----------- Output -----------\n{}\n\n---------- Expected ----------\n{}",
            output.join("\n"),
            expected_output.join("\n")
        );
    }

    Ok(())
}

#[allow(dead_code)]
fn generate_output_files(level: usize) -> Result<()> {
    let input_dir = format!("{}/level{}", INPUT_DIR, level);
    let output_dir = format!("{}/level{}", OUTPUT_DIR, level);

    // Create output directory if it doesn't exist
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create directory: {}", output_dir))?;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // Skip files that don't have valid filenames or are not .in files
        let file_name = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };
        if !file_name.starts_with(&format!("level{}_{}", level, "")) || !file_name.ends_with(".in") {
            continue;
        }

        let sub_level = file_name
            .strip_prefix(&format!("level{}_", level))
            .and_then(|s| s.strip_suffix(".in"))
            .filter(|&s| s != "example");

        if let Some(sub_level) = sub_level {
            let input_lines = read_lines(path.to_str().unwrap())?;
            let output = solve_level(level, input_lines);
            let output_path = build_path(level, sub_level, true);
            write_output_file(output_path, output)?;
        }
    }

    Ok(())
}

fn write_output_file(path: String, lines: Vec<String>) -> Result<()> {
    let mut file = fs::File::create(&path).with_context(|| format!("Could not create file at path: {}", path))?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    println!("Output written to: {}", path);
    Ok(())
}

#[allow(unused_variables)]
fn solve_level(level: usize, input_lines: Vec<String>) -> Vec<String> {
    #[allow(clippy::match_single_binding)]
    match level {
        // NOTE: uncomment levels
        // 1 => level1::solve(input_lines),
        // 2 => level2::solve(input_lines),
        // 3 => level3::solve(input_lines),
        // 4 => level4::solve(input_lines),
        // 5 => level5::solve(input_lines),
        // 6 => level6::solve(input_lines),
        // 7 => level7::solve(input_lines),
        _ => unreachable!()
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();
    let input_path = get_level_input_path(&args)?;
    let input_lines = read_lines(&input_path)?;
    let output = solve_level(args.level, input_lines);

    match args.sub_level {
        Some(_) => output.iter().for_each(|line| println!("{line}")),
        None => compare_output(output, args.level)?
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
