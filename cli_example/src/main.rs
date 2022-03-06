use dotenv::dotenv;
use miette::{IntoDiagnostic, Result};
use std::env;
use structopt::StructOpt;
use tracing::debug;

#[derive(StructOpt)]
#[structopt()]
pub struct Opt {
    /// Print debug info
    #[structopt(short, long)]
    verbose: bool,

    /// Foo identifier
    #[structopt(long, default_value = "bar")]
    foo: String,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// Say hello
    Hello { name: String },
    /// Say goodbye
    Goodbye,
}

pub fn init_foo() -> Result<()> {
    debug!("Init foo");
    let _token = env::var("FOO_AUTH")
        .into_diagnostic()?
        .parse::<u64>()
        .into_diagnostic()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let opt = Opt::from_args();

    let log_level = if opt.verbose { "DEBUG" } else { "INFO" };
    tracing_subscriber::fmt()
        // Allows to set a filter on spans and events based on directives in RUST_LOG env var
        // See https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
        .with_env_filter(tracing_subscriber::EnvFilter::new(log_level))
        .with_level(true)
        // Set the subscriber as the default.
        .init();

    init_foo()?;

    match &opt.cmd {
        Command::Hello { name } => {
            println!("Hello {}", name);
        }
        Command::Goodbye => {
            println!("Bye {}", opt.foo);
        }
    };

    Ok(())
}
