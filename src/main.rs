use clap::crate_authors;
use clap::{App, AppSettings, Arg};

use csskit::*;

fn main() {
    let long_version = crate::shadow::clap_version();
    let matches = App::new("csskit")
        .about("A toolkit for inspecting, improving and working with CSS.")
        .version(shadow::PKG_VERSION)
        .long_version(long_version.as_str())
        .author(crate_authors!())
        .after_help("https://github.com/wolves/csskit")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            App::new("selectors")
                .about("Work with CSS selectors")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .setting(AppSettings::ColoredHelp)
                .subcommand(
                    App::new("search")
                        .about("Searches target for CSS selectors")
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .setting(AppSettings::ColoredHelp)
                        .arg(
                            Arg::new("query")
                                .required(true)
                                .about("The selector query string"),
                        )
                        .arg(
                            Arg::new("target")
                                .required(true)
                                .about("The target file/s to search"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("selectors", selector_matches)) => match selector_matches.subcommand() {
            Some(("search", search_matches)) => {
                selectors::search_target(
                    search_matches.value_of("query").unwrap(),
                    search_matches.value_of("target").unwrap(),
                );
            }
            _ => unreachable!(),
        },
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
