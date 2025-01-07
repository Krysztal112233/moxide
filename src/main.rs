use std::{env, io, path::PathBuf};

use clap::{arg, value_parser, ArgAction, ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};
use human_panic::setup_panic;
use log::error;
use proj::MoxideProj;
use util::CreateType;

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
    setup_panic!();

    if let Err(env::VarError::NotPresent) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    let matches = app().get_matches();

    let result = match matches.subcommand() {
        Some(("build", cmd)) => subcommand_build(cmd).await,
        Some(("serve", cmd)) => subcommand_serve(cmd),
        Some(("new", cmd)) => subcommand_create(cmd),
        Some(("completion", cmd)) => subcommand_completion(cmd),
        _ => Ok(()),
    };

    if let Err(e) = result {
        error!("{e}")
    }

    Ok(())
}

fn app() -> Command {
    Command::new("moxide")
        .args([arg!(-p --property "Inject property").action(ArgAction::Append)])
        .subcommand(
            Command::new("build")
                .about("Build Moxide project")
                .args([arg!(--out "Output directory").action(ArgAction::Set)]),
        )
        .subcommand(
            Command::new("serve")
                .about("Serve Moxide with live reloading")
                .args([
                    arg!(--port "Listening port").default_value("8000"),
                    arg!(--addr "Listening addr").default_value("0.0.0.0"),
                ]),
        )
        .subcommand(
            Command::new("new")
                .about("Create new project/page/bundle")
                .args([
                    arg!(--name "Created with name")
                        .action(ArgAction::Set)
                        .index(1)
                        .required(true),
                    arg!(--type "Which you want to create")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(CreateType))
                        .default_value("page"),
                ]),
        )
        .subcommand(
            Command::new("completion")
                .about("Generate shell completion")
                .args([
                    arg!(--shell "Which shell's completion you want to generate")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(Shell)),
                ]),
        )
}

async fn subcommand_build(matches: &ArgMatches) -> anyhow::Result<()> {
    let output = matches
        .try_get_one::<String>("output")?
        .cloned()
        .unwrap_or("./output".to_owned());

    let proj_path = PathBuf::from_iter(["./manifest.toml"]);
    let mut proj = MoxideProj::try_new(proj_path)?;
    proj.set_output(output);
    proj.build().await?;

    Ok(())
}

fn subcommand_create(matches: &ArgMatches) -> anyhow::Result<()> {
    let name = matches.get_one::<String>("name").cloned().unwrap();

    match matches.get_one::<CreateType>("type") {
        Some(CreateType::Project) => {
            let proj = MoxideProj::create_proj(name)?;
            proj.create_page("HelloWorld")?;
        }
        Some(CreateType::Page) => {
            let proj_path = PathBuf::from_iter(["./manifest.toml"]);
            let proj = MoxideProj::try_new(proj_path)?;

            proj.create_page(name)?;
        }
        Some(CreateType::Bundle) => {}
        _ => {
            todo!()
        }
    };

    Ok(())
}

fn subcommand_serve(matches: &ArgMatches) -> anyhow::Result<()> {
    Ok(())
}

fn subcommand_completion(matches: &ArgMatches) -> anyhow::Result<()> {
    fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    }

    if let Some(generator) = matches.get_one::<Shell>("shell").copied() {
        let mut cli = app();
        print_completions(generator, &mut cli);
    }

    Ok(())
}
