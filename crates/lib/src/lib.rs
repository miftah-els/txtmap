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

/// Initialize txtmap in current working directory.
/// 
/// This will create the following files and directories:
/// - `map` (input directory)
/// - `terr` (output directory)
/// - `style.css`
/// - `.gitignore` (`terr` is ignored by default)
/// - `txtmap.toml` (config)
use std::fs;

pub fn init() -> std::io::Result<()> {
    fs::create_dir("./map")?;
    fs::create_dir("./terr")?;
    fs::write("./style.css", "/* your css here */\n")?;
    fs::write("./.gitignore", "/terr\n")?;
    fs::write("./txtmap.toml", "# your config here\n")?;
    Ok(())
}