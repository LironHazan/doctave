use bunt::termcolor::{ColorChoice, StandardStream};
use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("Doctave")
        .version(env!("CARGO_PKG_VERSION"))
        .about(
            "An opinionated static site generator designed specifically \
               for technical documentation.",
        )
        .arg(
            Arg::with_name("no-color")
                .long("no-color")
                .help("Disable terminal color output")
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a new project (start here!)")
                .arg(Arg::with_name("docs-dir").long("docs-dir").help(
                    "An optional custom root directory for your documentation. (Defaults to docs/)",
                ).takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds your site from the project's Markdown files")
                .arg(
                    Arg::with_name("release")
                        .long("release")
                        .help("Build the site in release mode"),
                )
                .arg(
                    Arg::with_name("allow-failed-checks")
                        .long("allow-failed-checks")
                        .help("Don't return an error if there are failed checks"),
                ),
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about(
                    "Starts a live reloading development server to serve your documentation site",
                )
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .short("p")
                        .takes_value(true)
                        .value_name("PORT")
                        .help(
                            "Port used to serve the documentation site. \
                             Must be a positive integer.",
                        )
                        .validator(|p| match p.parse::<u32>() {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e.to_string()),
                        }),
                ).arg(
                Arg::with_name("open")
                    .long("open")
                    .short("o")
                    .help("Opens the docs site on chrome")
            )
            ,
        )
        .get_matches();

    let result = match matches.subcommand() {
        ("init", Some(cmd)) => init(cmd),
        ("build", Some(cmd)) => build(cmd),
        ("serve", Some(cmd)) => serve(cmd),
        _ => Ok(()),
    };

    let mut out = if matches.value_of("no-color").is_some() {
        StandardStream::stdout(ColorChoice::Never)
    } else {
        StandardStream::stdout(ColorChoice::Auto)
    };

    if let Err(e) = result {
        bunt::writeln!(out, "{$red}ERROR:{/$} {}", e).unwrap();
        std::process::exit(1);
    }
}

fn init(cmd: &ArgMatches) -> doctave::Result<()> {
    let root_dir = std::env::current_dir().expect("Unable to determine current directory");
    let doc_root = cmd.value_of("docs-dir").map(|str| str.to_string());
    doctave::InitCommand::run(root_dir, !cmd.is_present("no-color"), doc_root)
}

fn build(cmd: &ArgMatches) -> doctave::Result<()> {
    let project_dir = doctave::config::project_root().unwrap_or_else(|| {
        println!("Could not find a doctave project in this directory, or its parents.");
        std::process::exit(1);
    });

    let mut config = doctave::Config::load(&project_dir)?;
    if cmd.is_present("release") {
        config.set_build_mode(doctave::BuildMode::Release);
    }

    if cmd.is_present("no-color") {
        config.disable_colors();
    }

    if cmd.is_present("allow-failed-checks") {
        config.set_allow_failed_checks();
    }

    doctave::BuildCommand::run(config)
}

fn serve(cmd: &ArgMatches) -> doctave::Result<()> {
    let project_dir = doctave::config::project_root().unwrap_or_else(|| {
        println!("Could not find a doctave project in this directory, or its parents.");
        std::process::exit(1);
    });

    let mut options = doctave::ServeOptions::default();
    let mut config = doctave::Config::load(&project_dir)?;

    if let Some(p) = cmd.value_of("port") {
        options.port = Some(p.parse::<u32>().unwrap());
    }

    options.open = cmd.is_present("open");

    if cmd.is_present("no-color") {
        config.disable_colors();
    }

    doctave::ServeCommand::run(options, config)
}
