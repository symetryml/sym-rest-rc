mod cli;
mod config;
mod commands;
mod auth;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Auto-load configuration before executing any command
    let config_result = config::Config::auto_load(cli.config);

    let result = match cli.command {
        Commands::Config => {
            // For the config command, show config info if loaded, or show error
            match config_result {
                Ok(path) => handle_config(&path),
                Err(e) => Err(e.into())
            }
        }
        _ => {
            // For all other commands, config must be loaded
            match config_result {
                Ok(_) => {
                    // Config loaded successfully, execute the command
                    match cli.command {
                        Commands::Config => unreachable!(),
                        Commands::Create(args) => {
                            commands::create_project::handle_create(args.name, args.r#type, args.params, args.hist).await
                        }
                        Commands::Learn(args) => {
                            if args.use_ws {
                                commands::learn_ws::handle_learn(args.project, args.file, args.types).await
                            } else {
                                commands::learn_rest::handle_learn(args.project, args.file, args.types).await
                            }
                        }
                        Commands::Build(args) => {
                            commands::build_rest::handle_build(
                                args.project,
                                args.name,
                                args.r#type,
                                args.targets,
                                args.inputs,
                                args.target_names,
                                args.input_names,
                                args.params
                            ).await
                        }
                        Commands::Predict(args) => {
                            if args.use_ws {
                                commands::predict_ws::handle_predict(args.project, args.model, args.df, args.file).await
                            } else {
                                commands::predict_rest::handle_predict(args.project, args.model, args.df, args.file).await
                            }
                        }
                        Commands::Job(args) => {
                            commands::job_rest::handle_job(args.id).await
                        }
                    }
                }
                Err(e) => Err(e.into())
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_config(loaded_from: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Configuration loaded from: {}", loaded_from);
    println!("Host: {}", config::Config::host());
    println!("Port: {}", config::Config::port());
    println!("User: {}", config::Config::user());
    println!("Secret Key: {}", config::Config::secretkey());
    println!("Use WebSocket for Learn: {}", config::Config::use_ws_for_learn());
    println!("Use WebSocket for Predict: {}", config::Config::use_ws_for_predit());
    Ok(())
}

