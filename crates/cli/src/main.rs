// `txtmap` is an inter-file parsing tool for `txtstar`, a custom plaintext format with internal and external links.
// Copyright (C) 2026  Miftah

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmds>
}

#[derive(Subcommand)]
enum Cmds {
    /// Initialize txtmap in current working directory
    Init
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let cli = Cli::parse();
    if let Some(cmd) = cli.cmd {
        match cmd {
            Cmds::Init => lib::init()?
        }
    }

    Ok(())
}
