use headhunter_bindings::{authentication::*, request::*, response::*, Client};
use std::fs::File;

#[derive(argh::FromArgs)]
/// CLI client for managing resumes on a job search site called "headhunter" (https://hh.ru)
struct Args {
    #[argh(subcommand)]
    command: MySubCommandEnum,
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum MySubCommandEnum {
    Auth(SubCommandAuth),
    Bump(SubCommandBump),
}

#[derive(argh::FromArgs)]
/// Gets access_token
#[argh(subcommand, name = "auth")]
struct SubCommandAuth {
    /// client_id from https://dev.hh.ru/admin
    #[argh(positional)]
    client_id: String,
    /// client_secret from https://dev.hh.ru/admin
    #[argh(positional)]
    client_secret: String,
    /// login for https://hh.ru
    #[argh(positional)]
    login: String,
    /// password for https://hh.ru
    #[argh(positional)]
    password: String,
}

#[derive(argh::FromArgs)]
/// Bumps resume, reads access_token from response.json
#[argh(subcommand, name = "bump")]
struct SubCommandBump {}

const RESPONSE_FILENAME: &str = "response.json";

async fn auth(
    SubCommandAuth {
        ref client_id,
        ref client_secret,
        ref login,
        ref password,
    }: SubCommandAuth,
) -> color_eyre::eyre::Result<()> {
    #[rustfmt::skip]
    let client = AuthenticationClient::new(
        ApplicationCredentials { client_id, client_secret },
        UserCredentials { login, password },
    );

    println!("Trying to get authorization_code using the Selenium WebDriver...");
    let authorization_code = client.get_authorization_code().await?;

    println!("Trying to get perform authentication...");
    let response = client.perform_authentication(&authorization_code).await?;

    println!("Saving data into {RESPONSE_FILENAME}...");
    serde_json::to_writer(&File::create(RESPONSE_FILENAME)?, &response)?;

    println!("{response:#?}");

    Ok(())
}

async fn bump() -> color_eyre::eyre::Result<()> {
    let UserOpenAuthorizationResponse {
        ref access_token, ..
    } = serde_json::from_reader(File::open(RESPONSE_FILENAME)?)?;

    let client = Client::new(access_token)?;

    let response = client.get(&MeRequest).await?;
    println!("Logged as {} with {}", response.auth_type, response.email);

    //TODO: refresh_token

    let response = client.get(&MineResumesRequest).await?;
    println!("Found {} resumes", response.found);

    for item in response.items {
        let url = url::Url::parse(&item.alternate_url)?;
        let resume_id = url
            .path_segments()
            .expect("failed to split URL")
            .nth(1)
            .expect("failed to get resume ID");

        println!("Processing resume with id {resume_id}");

        let ret = client
            .post_with_value(&PublishResumeRequest, resume_id)
            .await;

        if let Err(err) = ret {
            println!("{err:#?}")
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let args: Args = argh::from_env();

    color_eyre::install()?;

    match args.command {
        MySubCommandEnum::Auth(args) => auth(args).await,
        MySubCommandEnum::Bump(_) => bump().await,
    }
}
