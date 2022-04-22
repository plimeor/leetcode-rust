use super::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

const GRAPHQL_URL: &str = "https://leetcode-cn.com/graphql";
const QUESTION_QUERY_OPERATION: &str = "questionData";
const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
    questionId
    questionFrontendId
    title
    titleSlug
    content
    translatedContent
    difficulty
    codeSnippets {
      lang
      langSlug
      code
      __typename
    }
    stats
    hints
    status
    sampleTestCase
    exampleTestcases
    jsonExampleTestcases
    __typename
  }
}
"#;

pub async fn get_problem(problem_id: &String) -> Problem {
    let title_slug = get_problems::get_title_slug(&problem_id).await;

    let client = reqwest::Client::new();

    let raw_problem: RawProblem = client
        .post(GRAPHQL_URL)
        .json(&Query::query_problem(title_slug))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let question = raw_problem.data.question;

    Problem {
        id: question.question_frontend_id,
        title: question.title,
        title_slug: question.title_slug,
        difficulty: question.difficulty,
        content: question.translated_content,
        code: question
            .code_snippets
            .iter()
            .find(|snip| snip.lang_slug == "rust")
            .unwrap()
            .code
            .clone(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    pub fn query_problem(title_slug: String) -> Query {
        return Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    #[serde(rename = "questionId")]
    question_id: String,
    #[serde(rename = "questionFrontendId")]
    question_frontend_id: String,
    title: String,
    #[serde(rename = "titleSlug")]
    title_slug: String,
    content: String,
    #[serde(rename = "translatedContent")]
    translated_content: String,
    difficulty: String,
    #[serde(rename = "codeSnippets")]
    code_snippets: Vec<CodeSnippet>,
    #[serde(rename = "exampleTestcases")]
    example_test_cases: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeSnippet {
    lang: String,
    #[serde(rename = "langSlug")]
    lang_slug: String,
    code: String,
}

#[derive(Debug)]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub title_slug: String,
    pub content: String,
    pub difficulty: String,
    pub code: String,
}
