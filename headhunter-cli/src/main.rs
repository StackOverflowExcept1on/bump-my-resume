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
    let client = AuthenticationClient::new();

    let application = ApplicationCredentials {
        client_id,
        client_secret,
    };

    let user = UserCredentials { login, password };

    tracing::info!("Trying to get authorization_code using the Selenium WebDriver...");
    let authorization_code = client.get_authorization_code(&application, &user).await?;

    tracing::info!("Trying to get perform authentication...");
    let response = client
        .perform_authentication(&application, &authorization_code)
        .await?;

    tracing::info!("Saving data into {RESPONSE_FILENAME}...");
    serde_json::to_writer(&File::create(RESPONSE_FILENAME)?, &response)?;

    tracing::debug!("{response:#?}");

    Ok(())
}

async fn prepare_client<'a>() -> color_eyre::eyre::Result<Client> {
    let UserOpenAuthorizationResponse {
        access_token,
        ref refresh_token,
        ..
    } = serde_json::from_reader(File::open(RESPONSE_FILENAME)?)?;

    let client = Client::new(access_token)?;

    return match client.get(&MeRequest).await {
        Ok(_) => {
            tracing::info!("Using existing token...");
            Ok(client)
        }
        Err(_) => {
            tracing::info!("Looks like the token has expired, refreshing...");

            let authentication_client = AuthenticationClient::new();
            let response = authentication_client.refresh_token(refresh_token).await?;

            tracing::info!("Saving new token into {RESPONSE_FILENAME}...");
            serde_json::to_writer(&File::create(RESPONSE_FILENAME)?, &response)?;

            tracing::debug!("{response:#?}");

            Ok(Client::new(response.access_token)?)
        }
    };
}

async fn bump() -> color_eyre::eyre::Result<()> {
    let client = prepare_client().await?;

    let response = client.get(&MeRequest).await?;
    tracing::info!("Logged as {} with {}", response.auth_type, response.email);

    let response = client.get(&MineResumesRequest).await?;
    tracing::info!("Found {} resumes", response.found);

    for item in response.items {
        let url = url::Url::parse(&item.alternate_url)?;
        let resume_id = url
            .path_segments()
            .expect("failed to split URL")
            .nth(1)
            .expect("failed to get resume ID");

        tracing::info!("Processing resume with id {resume_id}");

        let ret = client
            .post_with_value(&PublishResumeRequest, resume_id)
            .await;

        if let Err(err) = ret {
            tracing::error!("{err:#?}")
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let args: Args = argh::from_env();

    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
            time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC),
            time::macros::format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
        ))
        .with_env_filter("headhunter_cli=trace")
        .init();

    match args.command {
        MySubCommandEnum::Auth(args) => auth(args).await,
        MySubCommandEnum::Bump(_) => bump().await,
    }
}
