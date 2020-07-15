use clap::Clap;

#[derive(Clone, Clap, Debug)]
#[clap(name = "twm")]
pub enum App {
    Info(InfoSubCommand),
}

#[derive(Clone, Clap, Debug)]
pub enum InfoSubCommand {
    Get {
        #[clap(name = "NAME")]
        name: String,
    },
    // Set {
    //     #[clap(name = "NAME")]
    //     name: String,
    //
    //     #[clap(name = "VALUE")]
    //     value: String,
    // },
}
