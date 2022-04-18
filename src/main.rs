extern crate core;

use std::io::prelude::*;
use std::{fs, path};
mod common;

#[allow(unused)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
mod problems;

fn main() {
    init_problem(349, String::from("intersection-of-two-arrays")).unwrap();
}

fn init_problem(problem_id: u32, problem_title: String) -> Result<(), std::io::Error> {
    let solution_name = format!(
        "no{:04}_{}",
        problem_id,
        problem_title.clone().replace("-", "_")
    );
    let solution_path = format!("./src/problems/{}.rs", &solution_name);

    if path::Path::new(&solution_path).exists() {
        return Ok(());
    }

    let solution_content = format!(
        "// https://leetcode-cn.com/problems/{}/\nstruct Solution;\n

#[test]
fn test(){{
}}
",
        problem_title
    );

    fs::write(&solution_path, solution_content)?;

    let mut mod_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./src/problems.rs")?;

    writeln!(mod_file, "mod {};", &solution_name)?;

    Ok(())
}
