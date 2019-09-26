extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("makectl")
        .version("1.0")
        .author("Bruno Rocha")
        .about("Generate and Manage targets in your makefiles.")
        .subcommand(SubCommand::with_name("add")
            .about("Add template that you want to use")
            .arg(Arg::with_name("template")
                .short("t")
                .long("template")
                .value_name("TEMPLATE")
                .help("Template you want to use")
                .required(true)
                .takes_value(true)
                .multiple(true)
            )
        )
        .get_matches();

    //makectl add --template=python-clean
    if let Some(args) = matches.subcommand_matches("add") {
        for template_value in args.values_of("template").unwrap() {
            println!("Template value: {}", template_value);
        }
    }

    println!("Hello, world!");
}
