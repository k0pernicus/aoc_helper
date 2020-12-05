use clap::{App, Arg, ArgMatches};

/// Wraps a clap App
pub struct AOCApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> AOCApp<'a, 'b> {
    /// Initializes and returns an AOCApp
    pub fn new(app_name: &'b str, version: &'b str, author: &'b str) -> Self {
        AOCApp {
            app: App::new(app_name).version(version).author(author).arg(
                Arg::with_name("input")
                    .index(1)
                    .default_value("input.txt")
                    .help("Run the input in a dedicated input file"),
            ),
        }
    }
    /// Useful to customize a clap App
    pub fn add_argument(
        mut self,
        arg_name: &'a str,
        long_arg_name: &'b str,
        help: &'b str,
    ) -> Self {
        self.app = self
            .app
            .arg(Arg::with_name(arg_name).long(long_arg_name).help(help));
        self
    }

    /// Returns the matches
    pub fn build(self) -> AOCAppMatches<'a> {
        AOCAppMatches {
            matches: self.app.get_matches(),
        }
    }
}

/// Keep in memory the user arguments (and matches)
pub struct AOCAppMatches<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> AOCAppMatches<'a> {
    /// Default function to get the input file
    pub fn get_input_filename(&self) -> Option<&str> {
        self.matches.value_of("input")
    }
}
