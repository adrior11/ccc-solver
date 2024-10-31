use std::fs;
use std::io::Write;
use anyhow::{Context, Result};
use clap::Parser;

// NOTE: uncomment levels
// mod level1;
// mod level2;
// mod level3;
// mod level4;
// mod level5;
// mod level6;
// mod level7;

#[derive(Parser)]
struct Cli {
    level: usize,
    sub_level: Option<usize>,
}

fn read_lines(path: &str) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read file at path: {}", path))?;
    let lines = content.lines().map(|line| line.to_string()).collect();
    Ok(lines)
}

fn get_input_path(args: &Cli) -> Result<String> {
    let sub_level = match args.sub_level {
        Some(sub_level) => sub_level.to_string(),
        None => "example".to_string()
    };

    let path = format!(
        "src/input/level{}/level{}_{}.in",
        args.level,
        args.level,
        sub_level
    );
    
    Ok(path)
}

#[allow(dead_code)]
fn get_output_path(level: usize, sub_level: &str) -> String {
    format!("src/output/level{}/level{}_{}.out", level, level, sub_level)
}

fn get_example_output_path(level: usize) -> String {
    format!("src/input/level{}/level{}_example.out", level, level)
}

fn print_vec_str(vec: &[String]) {
    vec.iter().for_each(|line| println!("{line}"));
}

fn compare_output(out: Vec<String>, level: usize) -> Result<()> {
    let example_out_path = get_example_output_path(level);
    let result = read_lines(&example_out_path)?;

    if out == result {
        println!("Solution matches example!");
        generate_output_files(level)?;
    } else {
        let line = "-".repeat(20);
        println!("\n{}- Output -{}", line, line);
        print_vec_str(&out);
        println!("\n{} Expected {}", line, line);
        print_vec_str(&result);
    }

    Ok(())
}

#[allow(unused_variables)]
fn run_level(level: usize, input_lines: Vec<String>) -> Vec<String> {
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

#[allow(dead_code)]
fn generate_output_files(level: usize) -> Result<()> {
    let input_dir = format!("src/input/level{}", level);
    let output_dir = format!("src/output/level{}", level);

    // Check if output directory exists
    fs::create_dir_all(&output_dir).with_context(|| format!("Failed to create directory: {}", output_dir))?;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if file_name_str.starts_with(&format!("level{}_{}", level, "")) && file_name_str.ends_with(".in") {
                    let sub_level = file_name_str
                        .trim_start_matches(&format!("level{}_{}", level, ""))
                        .trim_end_matches(".in");

                    if sub_level == "example" {
                        continue;
                    }

                    let input_lines = read_lines(path.to_str().unwrap())?;
                    let output = run_level(level, input_lines);

                    let output_path = get_output_path(level, sub_level);
                    write_output_file(output_path, output)?;
                }
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn write_output_file(path: String, lines: Vec<String>) -> Result<()> {
    let mut file = fs::File::create(&path).with_context(|| format!("Could not create file at path: {}", path))?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    println!("Output written to: {}", path);
    Ok(())
}

fn run() -> Result<()> {
    let args = Cli::parse();
    let input_path = get_input_path(&args)?;
    let input_lines = read_lines(&input_path)?;
    let out = run_level(args.level, input_lines);

    match args.sub_level {
        Some(_) => out.iter().for_each(|line| println!("{line}")),
        None => compare_output(out, args.level)?
    }

    Ok(())
}

fn main() {
    let _test = "String";
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
