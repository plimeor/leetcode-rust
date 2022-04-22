mod get_problem;
mod get_problems;

use get_problem::*;
use std::io::prelude::*;
use std::{fs, path};

pub async fn create_problem(problem_id: String) -> Result<(), String> {
    let problem = get_problem(&problem_id).await;
    let problem_name = format!("no{}_{}", problem_id, problem.title_slug.replace("-", "_"));
    let problem_path = format!("./problems/src/problems/{}.rs", &problem_name);
    let problem_desc = build_desc(&problem.content);

    if path::Path::new(&problem_path).exists() {
        return Ok(());
    }

    let problem_content = format!(
        "// https://leetcode-cn.com/problems/{}/

/**
 * {}
 *
 * {}
*/

struct Solution;

{}

#[test]
fn test(){{
}}
",
        problem.title_slug, problem.title, problem_desc, problem.code
    );

    fs::write(&problem_path, problem_content).unwrap();

    let mut mod_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("./problems/src/problems.rs")
        .unwrap();

    writeln!(mod_file, "mod {};", &problem_name).unwrap();

    Ok(())
}

fn build_desc(content: &str) -> String {
    content
        .replace(" ", "")
        .replace("​", "")
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("</p>", "")
        .replace("<p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", "")
        .replace("<code>", "")
        .replace("</code>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<sub>", "")
        .replace("</sub>", "")
        .replace("</sup>", "")
        .replace("<sup>", "^")
        .replace("&nbsp;", " ")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&minus;", "-")
        .replace("&#39;", "'")
        .replace("\n\n", "\n")
        .replace("\n", "\n * ")
}
