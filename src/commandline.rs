use clap::{App, Arg, ArgMatches};

pub struct AOCApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> AOCApp<'a, 'b> {
    pub fn new(app_name: &'b str, version: &'b str, author: &'b str) -> Self {
        AOCApp {
            app: App::new(app_name).version(version).author(author).arg(
                Arg::with_name("input-file")
                    .index(1)
                    .help("Run the input in a dedicated input file"),
            ),
        }
    }

    pub fn add_argument(
        mut self,
        arg_name: &'a str,
        long_arg_name: &'b str,
        short_arg_name: &'b str,
        required: bool,
        takes_value: bool,
        help: &'b str,
    ) -> Self {
        self.app = self.app.arg(
            Arg::with_name(arg_name)
                .long(long_arg_name)
                .short(short_arg_name)
                .required(required)
                .takes_value(takes_value)
                .help(help),
        );
        self
    }

    pub fn build(self) -> AOCAppMatches<'a> {
        AOCAppMatches {
            matches: self.app.get_matches(),
        }
    }
}

impl<'a, 'b> Default for AOCApp<'a, 'b> {
    fn default() -> Self {
        AOCApp::new(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
    }
}

#[derive(Debug)]
pub struct AOCAppMatches<'a> {
    matches: ArgMatches<'a>,
}

impl<'a> AOCAppMatches<'a> {
    pub fn get_input_filename(&self) -> Option<&str> {
        self.get_value_of("input")
    }
    pub fn get_value_of(&self, arg_name: &str) -> Option<&str> {
        self.matches.value_of(arg_name)
    }
}

#[macro_export]
macro_rules! get_app_args {
    () => {
        AOCApp::default().build()
    };
}
