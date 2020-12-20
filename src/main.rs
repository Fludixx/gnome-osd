use std::error::Error;

use shell::GnomeShell;
use structopt::StructOpt;

mod shell;

#[derive(Debug, StructOpt)]
#[structopt(name = "gnome-osd", about = "Exectutes the ShowOSD method from the org.gnome.Shell interface.")]
struct Opt {
    #[structopt(long)]
    pub icon: String,
    #[structopt(long)]
    pub label: Option<String>,
    #[structopt(long)]
    pub level: Option<f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let session = GnomeShell::new_session();
    session.show_osd(
        opt.icon,
        opt.label,
        opt.level,
    )
}
