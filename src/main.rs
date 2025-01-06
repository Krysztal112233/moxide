use std::env;

use clap::{arg, ArgAction, ArgMatches, Command};
use log::error;
use proj::MoxideProj;

mod builder;
mod error;
mod manifest;
mod mkentry;
mod proj;
mod property;
mod render;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(env::VarError::NotPresent) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let matches = app();

    let result = match matches.subcommand() {
        Some(("build", cmd)) => subcommand_build(cmd),
        Some(("serve", cmd)) => subcommand_serve(cmd),
        Some(("create", cmd)) => subcommand_create(cmd),
        _ => Ok(()),
    };

    if let Err(e) = result {
        error!("{e}")
    }

    Ok(())
}

fn app() -> ArgMatches {
    Command::new("moxide")
        .args([arg!(-p --property "Inject property").action(ArgAction::Append)])
        .subcommand(
            Command::new("build")
                .about("Build Moxide project")
                .args([arg!(--out "Output directory").default_value("./output")]),
        )
        .subcommand(
            Command::new("serve")
                .about("Serve Moxide with live reloading")
                .args([
                    arg!(--port "Listening port").default_value("8000"),
                    arg!(--addr "Listening addr").default_value("0.0.0.0"),
                    arg!(--tmp  "Temporary directory").default_value("./tmp"),
                ]),
        )
        .subcommand(
            Command::new("create")
                .about("Create new project/page/bundle")
                .args([
                    arg!(--name "Created with name")
                        .action(ArgAction::Set)
                        .default_value("default"),
                    arg!(--type "Which you want to create")
                        .action(ArgAction::Set)
                        .value_parser(["project", "page", "bundle"]),
                ]),
        )
        .get_matches()
}

fn subcommand_build(matches: &ArgMatches) -> anyhow::Result<()> {
    Ok(())
}

fn subcommand_create(matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.get_one::<String>("type").cloned() {
        Some(val) if val == *"project" => {
            let proj_name = matches.get_one::<String>("name").cloned().unwrap();
            let proj = MoxideProj::create_proj(proj_name)?;
            proj.new_page("HelloWorld")?;
        }
        Some(val) if val == *"page" => {}
        Some(val) if val == *"bundle" => {}
        _ => {}
    };

    Ok(())
}

fn subcommand_serve(matches: &ArgMatches) -> anyhow::Result<()> {
    Ok(())
}
