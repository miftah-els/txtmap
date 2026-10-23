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

use std::{
    collections::HashMap, 
    fs, 
    iter::{IntoIterator, Iterator}, 
    path::{Path, PathBuf}
};

/// Initialize txtmap in current working directory.
/// 
/// This will create the following files and directories:
/// - `map` (input directory)
/// - `terr` (output directory)
/// - `style.css`
/// - `.gitignore` (`terr` is ignored by default)
/// - `txtmap.toml` (config)
pub fn init() -> std::io::Result<()> {
    fs::create_dir("./map")?;
    fs::create_dir("./terr")?;
    fs::write("./style.css", "/* your css here */\n")?;
    fs::write("./.gitignore", "/terr\n")?;
    fs::write("./txtmap.toml", "# your config here\n")?;
    Ok(())
}

type Backlinks<'a> = Vec<&'a Path>;
struct BacklinkStore<'a> {
    store: HashMap<&'a Path, Backlinks<'a>>
}
impl<'a> BacklinkStore<'a> {
    fn new() -> BacklinkStore<'a> {
        BacklinkStore { store: HashMap::new() }
    }

    fn load_txtstars(&mut self, txtstars: &'a TxtStars) {
        for (loc, links) in txtstars.into_iter() {
            for current in links {
                if let Some(backlinks) = self.store.get_mut(&current) {
                    (*backlinks).push(&loc);
                } else {
                    self.store.insert(current, vec![&loc]);
                }
            }
        }
    }
}

struct TxtStars {
    data: Vec<(PathBuf, Vec<PathBuf>)>
}
impl<'a> IntoIterator for &'a TxtStars {
    type Item = (&'a Path, Vec<&'a Path>);
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self
            .data
            .iter()
            .map(
                |(p, v)| 
                (
                    Path::new(p), 
                    v.iter().map(|vp| Path::new(vp)).collect()
                )
            )
            .collect::<Vec<Self::Item>>()
            .into_iter()
    }
}

fn list_txtstars() -> std::io::Result<TxtStars> {
    todo!()
}

pub fn build() -> std::io::Result<()> {
    let mut blstore = BacklinkStore::new();

    let txtstars = list_txtstars()?;
    
    let _ = blstore.load_txtstars(&txtstars);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_backlinks_store_loading1<'a>() {
        let p_main = PathBuf::from("./main.txtstar");
        let p_sec = PathBuf::from("./sec.txtstar");
        let p_third = PathBuf::from("./third.txtstar");

        let data = vec![
            (
                p_main.clone(),
                vec![
                    p_sec.clone(),
                    p_third.clone(),
                ]
            ),
            (
                p_sec.clone(),
                vec![
                    p_third.clone()
                ]
            )
        ];
        let txtstars = TxtStars { data };

        let mut blstore = BacklinkStore::new();
        blstore.load_txtstars(&txtstars);

        let correct: HashSet<(PathBuf, Vec<PathBuf>)> = HashSet::from([
            (p_sec.clone(), vec![p_main.clone()]),
            (p_third.clone(), vec![p_main.clone(), p_sec.clone()])
        ]);

        let mut real: HashSet<(PathBuf, Vec<PathBuf>)> = HashSet::new();
        for (ip, iv) in blstore.store.iter() {
            real.insert(
                (
                    PathBuf::from(ip), 
                    iv.into_iter().map(|p| PathBuf::from(p)).collect()
                )
            );
        }

        assert_eq!(correct, real)
    }
}