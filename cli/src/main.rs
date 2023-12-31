use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    // #[arg(short, long)]
    // name: String,
    //
    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

pub fn main() {
    let args = Args::parse();

    // println!("debug:{}", shadow_rs::is_debug()); // check if this is a debug build. e.g 'true/false'
    // println!("branch:{}", shadow_rs::branch()); // get current project branch. e.g 'master/develop'
    // println!("tag:{}", shadow_rs::tag()); // get current project tag. e.g 'v1.3.5'
    // println!("git_clean:{}", shadow_rs::git_clean()); // get current project clean. e.g 'true/false'
    // println!("git_status_file:{}", shadow_rs::git_status_file()); // get current project statue file. e.g '  * examples/builtin_fn.rs (dirty)'

    // println!("{}", build::VERSION); //print version const
    // println!("{}", build::CLAP_LONG_VERSION); //print CLAP_LONG_VERSION const
    // println!("{}", build::BRANCH); //master
    // println!("{}", build::SHORT_COMMIT);//8405e28e
    // println!("{}", build::COMMIT_HASH);//8405e28e64080a09525a6cf1b07c22fcaf71a5c5
    // println!("{}", build::COMMIT_DATE);//2021-08-04 12:34:03 +00:00
    // println!("{}", build::COMMIT_AUTHOR);//baoyachi
    // println!("{}", build::COMMIT_EMAIL);//xxx@gmail.com

    // println!("{}", build::BUILD_OS);//macos-x86_64
    // println!("{}", build::RUST_VERSION);//rustc 1.45.0 (5c1f21c3b 2020-07-13)
    // println!("{}", build::RUST_CHANNEL);//stable-x86_64-apple-darwin (default)
    // println!("{}", build::CARGO_VERSION);//cargo 1.45.0 (744bd1fbb 2020-06-15)
    // println!("{}", build::PKG_VERSION); //0.3.13
    // println!("{}", build::CARGO_TRs  EE); //like command:cargo tree
    // println!("{}", build::CARGO_MANIFEST_DIR); // /User/baoyachi/shadow-rs/ |
    //
    // println!("{}", build::PROJECT_NAME);//shadow-rs
    // println!("{}", build::BUILD_TIME);//2020-08-16 14:50:25
    // println!("{}", build::BUILD_RUST_CHANNEL);//debug
    // println!("{}", build::GIT_CLEAN);//false
    // println!("{}", build::GIT_STATUS_FILE);//* src/pg_functions (dirty)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
