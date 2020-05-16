use colour::{green_ln, red_ln};
use human_panic::setup_panic;
use structopt::StructOpt;

use dezoomify_rs::{Arguments, dezoomify};

#[tokio::main]
async fn main() {
    setup_panic!();
    let has_args = std::env::args_os().count() > 1;
    let mut has_errors = false;
    let args: Arguments = Arguments::from_args();
    init_log(&args);

    loop {
        match dezoomify(&args).await {
            Err(err) => {
                red_ln!("ERROR {}", err);
                has_errors = true;
            }
            Ok(saved_as) => {
                green_ln!("Image successfully saved to '{}' (current working directory: {})",
                         saved_as.to_string_lossy(),
                         std::env::current_dir()
                             .map(|p| p.to_string_lossy().to_string())
                             .unwrap_or_else(|_e| "unknown".into())
                );
            }
        }
        if has_args {
            // Command-line invocation
            break;
        }
    }
    if has_errors {
        std::process::exit(1);
    }
}

fn init_log(args: &Arguments) {
    let env = env_logger::Env::new().default_filter_or(&args.logging);
    env_logger::init_from_env(env);
}