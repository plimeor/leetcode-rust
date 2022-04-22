use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

const PROBLEMS_URL: &str = "https://leetcode-cn.com/api/problems/algorithms/";

pub async fn get_title_slug(problem_id: &String) -> String {
    get_problems()
        .await
        .stat_status_pairs
        .iter()
        .find(|status| &status.stat.frontend_question_id == problem_id)
        .unwrap()
        .stat
        .question_title_slug
        .clone()
}

pub async fn get_problems() -> Problems {
    reqwest::get(PROBLEMS_URL)
        .await
        .unwrap()
        .json::<Problems>()
        .await
        .unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    stat_status_pairs: Vec<Status>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__title")]
    question_title: String,
    #[serde(rename = "question__title_slug")]
    question_title_slug: String,
    frontend_question_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}
