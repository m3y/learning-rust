use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, ArgSettings, Clap};
use serde_json::json;
use serde_query::Deserialize;

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(), version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!()
)]
struct Opts {
    /// github token
    #[clap(
        long,
        env = "GITHUB_TOKEN",
        setting = ArgSettings::HideEnvValues,
    )]
    github_token: String,

    /// target pull request url
    url: String,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    #[query(".data.repository.name")]
    repository_name: String,
    #[query(".data.repository.pullRequest.number")]
    number: i32,
    #[query(".data.repository.pullRequest.state")]
    state: String,
}

impl PullRequest {
    fn to_message(&self) -> String {
        format!(
            "{} PR#{} is {}",
            &self.repository_name, &self.number, &self.state
        )
    }

    fn to_url(&self) -> String {
        format!(
            "https://github.com/Pay-Baymax/{}/pull/{}",
            &self.repository_name, &self.number
        )
    }
}

async fn query(owner: &str, name: &str, prnum: i32, github_token: &str) -> Result<PullRequest> {
    let client = reqwest::Client::builder()
        .user_agent("rust reqwest")
        .build()?;
    let query = r#"
        query ($owner: String!, $name: String!, $prnum: Int!) {
          repository(name: $name, owner: $owner) {
            name
            pullRequest(number: $prnum) {
              state
              number
            }
          }
        }"#;

    let request = json!({
        "query": query,
        "variables": {
            "owner": owner,
            "name": name,
            "prnum": prnum,
        }
    });

    Ok(client
        .post("https://api.github.com/graphql")
        .bearer_auth(&github_token)
        .json(&request)
        .send()
        .await?
        .json::<PullRequest>()
        .await?)
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    let tokenize_url = opts.url.split('/').collect::<Vec<_>>();
    if tokenize_url.len() != 7 {
        eprintln!("This URL is invalid: {}", opts.url);
        std::process::exit(1);
    }
    let (owner, name, prnumber) = (tokenize_url[3], tokenize_url[4], tokenize_url[6]);
    let prnum: i32 = prnumber.parse().unwrap_or_else(|_| {
        eprintln!("This URL is invalid: {}", opts.url);
        std::process::exit(1);
    });

    let response = query(owner, name, prnum, &opts.github_token).await;
    match &response {
        Err(e) => {
            eprintln!("{:#?}", e);
            std::process::exit(1);
        }
        Ok(pull_request) => {
            match &*pull_request.state {
                "OPEN" => {
                    eprintln!("{}", pull_request.to_url());
                }
                _ => {
                    println!("{}", pull_request.to_message());
                }
            }
            std::process::exit(exitcode::OK);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pull_request_to_message() {
        let pr = PullRequest {
            repository_name: "test_repository".to_string(),
            number: 10,
            state: "MERGED".to_string(),
        };
        assert_eq!("test_repository PR#10 is MERGED", pr.to_message());
    }
}
