mod cli;
mod config;
mod commands;
mod auth;
mod common;

use clap::Parser;
use cli::{Cli, Commands, ProjectCommands, ModelCommands};

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
                        Commands::Project(project_cmd) => {
                            match project_cmd.command {
                                ProjectCommands::Create(args) => {
                                    commands::projects::create::handle_create(args.name, args.r#type, args.params, args.hist).await
                                }
                                ProjectCommands::Delete(args) => {
                                    commands::projects::delete::handle_delete(args.project).await
                                }
                            }
                        }
                        Commands::Model(model_cmd) => {
                            match model_cmd.command {
                                ModelCommands::Build(args) => {
                                    commands::models::build::handle_build(
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
                                ModelCommands::Delete(args) => {
                                    commands::models::delete::handle_delete(args.project, args.model).await
                                }
                                ModelCommands::Autoselect(args) => {
                                    commands::models::autoselect::handle_autoselect(
                                        args.project,
                                        args.model,
                                        args.task,
                                        args.val_file,
                                        args.val_df,
                                        args.targets,
                                        args.inputs,
                                        args.target_names,
                                        args.input_names,
                                        args.params
                                    ).await
                                }
                                ModelCommands::Predict(args) => {
                                    if args.use_ws {
                                        commands::models::predict_ws::handle_predict(args.project, args.model, args.df, args.file).await
                                    } else {
                                        commands::models::predict_rest::handle_predict(args.project, args.model, args.df, args.file).await
                                    }
                                }
                            }
                        }
                        Commands::Learn(args) => {
                            if args.use_ws {
                                commands::learn_ws::handle_learn(args.project, args.file, args.types).await
                            } else {
                                commands::learn_rest::handle_learn(args.project, args.file, args.types).await
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

