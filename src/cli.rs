use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sym-rest-rc")]
#[command(about = "REST client for machine learning operations", long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(long, global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display the currently loaded configuration
    Config,
    /// Create a new project using REST API
    Create(CreateArgs),
    /// Learn or push data to a project using REST API or WebSocket
    Learn(LearnArgs),
    /// Create a new machine learning model using REST API
    Build(BuildArgs),
    /// Make predictions with a model using WebSocket
    Predict(PredictArgs),
    /// Check the status of an asynchronous job
    Job(JobArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    /// Name of the project
    #[arg(long)]
    pub name: String,
    /// Project type
    #[arg(long)]
    pub r#type: String,
    /// Additional parameters (key=value pairs, comma-separated)
    #[arg(long)]
    pub params: Option<String>,
    /// Enable histogram
    #[arg(long, default_value_t = false)]
    pub hist: bool,
}

#[derive(Args)]
pub struct LearnArgs {
    /// Project name/ID
    #[arg(long)]
    pub project: String,
    /// Path to the dataset file
    #[arg(long)]
    pub file: String,
    /// Data types (comma-separated, e.g., "C,C,C,B,B")
    #[arg(long)]
    pub types: String,
    /// Use WebSocket for communication
    #[arg(long, default_value_t = false)]
    pub use_ws: bool,
}

#[derive(Args)]
pub struct BuildArgs {
    /// Project name
    #[arg(long)]
    pub project: String,
    /// Name of the model
    #[arg(long)]
    pub name: String,
    /// Model type (algo)
    #[arg(long)]
    pub r#type: String,
    /// Target column IDs (comma-separated, e.g., "13" or "12,13")
    #[arg(long)]
    pub targets: Option<String>,
    /// Input column IDs (comma-separated, e.g., "0,1,2,3")
    #[arg(long)]
    pub inputs: Option<String>,
    /// Target attribute names (comma-separated, e.g., "Iris_setosa")
    #[arg(long)]
    pub target_names: Option<String>,
    /// Input attribute names (comma-separated, e.g., "sepal_length,sepal_width,petal_length,petal_width")
    #[arg(long)]
    pub input_names: Option<String>,
    /// Model parameters (key=value pairs, comma-separated)
    #[arg(long)]
    pub params: Option<String>,
}

#[derive(Args)]
pub struct PredictArgs {
    /// Project name
    #[arg(long)]
    pub project: String,
    /// Model name
    #[arg(long)]
    pub model: String,
    /// Dataframe in JSON format
    #[arg(long)]
    pub df: Option<String>,
    /// Path to the data file
    #[arg(long)]
    pub file: Option<String>,
    /// Use WebSocket for communication
    #[arg(long, default_value_t = false)]
    pub use_ws: bool,
}

#[derive(Args)]
pub struct JobArgs {
    /// Job ID to check status
    #[arg(long)]
    pub id: String,
}
