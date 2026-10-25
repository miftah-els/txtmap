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
    ffi::OsStr, 
    fs, 
    iter::{IntoIterator, Iterator}, 
    path::{Path, PathBuf}
};

const CWD: &'static str = ".";
const MAP_PATH: &'static str = "./map";
const TERR_PATH: &'static str = "./terr";
const STYLE_PATH: &'static str = "./styles.css";
const CONFIG_PATH: &'static str = "./txtmap.toml";

const TXTSTAR_EXT: &'static str = "txtstar";

/// Initialize txtmap in current working directory.
/// 
/// This will create the following files and directories:
/// - `map` (input directory)
/// - `terr` (output directory)
/// - `style.css`
/// - `.gitignore` (`terr` is ignored by default)
/// - `txtmap.toml` (config)
pub fn init(root_dir: Option<&Path>) -> std::io::Result<()> {
    let root = if let Some(root) = root_dir {
        root
    } else {
        Path::new(CWD)
    };
    
    let fpath = |subpath: &str| root.join(subpath);

    fs::create_dir(fpath(MAP_PATH))?;
    fs::create_dir(fpath(TERR_PATH))?;
    fs::write(fpath(STYLE_PATH), "/* your css here */\n")?;
    fs::write(fpath("./.gitignore"), "/terr\n")?;
    fs::write(fpath(CONFIG_PATH), "# your config here\n")?;
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

#[derive(Debug, Hash, PartialEq, Eq)]
struct ListedPath {
    path: PathBuf,
    typ: ListedPathType
}
impl From<PathBuf> for ListedPath {
    /// Return value should only contain `ListedPathType::File(ListedFileType::Txtstar)` (`X`)
    /// if the extension of the given path is exactly `TXTSTAR_EXT`.
    /// 
    /// This means that when `value.extension()` returns `None`,
    /// it should not contain `X`.
    fn from(value: PathBuf) -> Self {
        let typ = if value.is_dir() {
            ListedPathType::Dir
        } else {
            if &value.extension().unwrap_or(OsStr::new("")) == &OsStr::new(TXTSTAR_EXT) {
                ListedPathType::File(
                    ListedFileType::Txtstar
                )
            } else {
                ListedPathType::File(
                    ListedFileType::Other
                )
            }
        };
        
        ListedPath::new(value, typ)
    }
}
impl ListedPath {
    fn new(path: PathBuf, typ: ListedPathType) -> ListedPath {
        ListedPath { path, typ }
    }
    fn pathbuf(self) -> PathBuf {
        self.path
    }
    fn path(&self) -> &Path {
        &self.path
    }
    fn typ(&self) -> &ListedPathType {
        &self.typ
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum ListedPathType {
    File(ListedFileType),
    Dir
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum ListedFileType {
    Txtstar,
    Other
}

/// todo
struct StructuredDirList {}

struct DirLister {
    /// A linear list of all paths in the dir.
    dirlist: Vec<ListedPath>
}
impl DirLister {
    fn new() -> Self {
        Self { dirlist: Vec::new() }
    }

    fn walkdir(path: &Path) -> std::io::Result<Vec<ListedPath>> {
        let mut paths: Vec<ListedPath> = Vec::new();

        let dir_read = fs::read_dir(path)?;
        for entry in dir_read {
            let entry = entry?;
            let listed_path = ListedPath::from(entry.path());

            match listed_path.typ() {
                ListedPathType::File(_) => {},
                ListedPathType::Dir => {
                    paths.extend(Self::walkdir(listed_path.path())?);
                }
            }
            paths.push(listed_path);
        }
        
        Ok(paths)
    }

    fn make_dirlist(&mut self) -> std::io::Result<()> {
        self.dirlist = Self::walkdir(Path::new(CWD))?;
        Ok(())
    }

    fn list_txtstars(&self) -> std::io::Result<TxtStars> {
        let _ = self.dirlist;
        todo!()
    }
}

pub fn build() -> std::io::Result<()> {
    let mut blstore = BacklinkStore::new();

    let mut dir_lister = DirLister::new();
    dir_lister.make_dirlist()?;
    let txtstars = dir_lister.list_txtstars()?;

    let _ = blstore.load_txtstars(&txtstars);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet};

    fn init_on_tempdir() -> tempfile::TempDir {
        let root_dir = tempfile::tempdir().expect("temp dir creation");
        
        init(
            Some(root_dir.path())
        ).expect("successful init");

        root_dir
    }

    fn new_dummy_listed_path_type(is_dir: bool) -> ListedPathType {
        if is_dir {
            ListedPathType::Dir
        } else {
            ListedPathType::File(ListedFileType::Other)
        }
    }

    fn new_dummy_listed_path(path: PathBuf, is_dir: bool) -> ListedPath {
        ListedPath::new(
            path,
            new_dummy_listed_path_type(is_dir)
        )
    }

    fn dir_entry_to_dummy_listed_path(entry: fs::DirEntry) -> ListedPath {
        new_dummy_listed_path(
            entry.path(),
            entry.metadata().expect("valid metadata").is_dir()
        )
    }

    fn correct_init_list(root: &Path) -> HashSet<ListedPath> {
        let fpath = |subpath: &str| root.join(subpath);

        HashSet::from([
            new_dummy_listed_path(fpath(MAP_PATH), true),
            new_dummy_listed_path(fpath(TERR_PATH), true),
            new_dummy_listed_path(fpath(STYLE_PATH), false),
            new_dummy_listed_path(fpath("./.gitignore"), false),
            new_dummy_listed_path(fpath(CONFIG_PATH), false)
        ])
    }

    #[test]
    fn test_init() {
        let root_dir = init_on_tempdir();

        let dir_read = fs::read_dir(root_dir.path()).expect("dir read");
        let list: Vec<ListedPath> = dir_read
            .map(|r| r.expect("valid entry"))
            .map(|entry| dir_entry_to_dummy_listed_path(entry))
            .collect();

        let root = root_dir.path();

        let correct_list = correct_init_list(root);

        let mut actual_list = HashSet::new();
        for item in list {
            if !actual_list.insert(item) {
                panic!("duplicate filepath");
            }
        }

        assert_eq!(actual_list, correct_list)
    }

    #[test]
    fn test_backlinks_store_loading0<'a>() {
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

    #[test]
    fn test_listed_path_from_pathbuf_on_txtstar() {
        let dir = tempfile::tempdir().expect("temp dir creation");
        
        let fname = "testfile.txtstar";
        
        let path = dir.path().join(fname);

        let _ = fs::File::create(&path).expect("file creation");

        let actual_lpath = ListedPath::from(path.clone());
        
        let correct_lpath = ListedPath::new(
            path,
            ListedPathType::File(
                ListedFileType::Txtstar
            )
        );

        assert_eq!(actual_lpath, correct_lpath)
    }
    #[test]
    fn test_listed_path_from_pathbuf_on_non_txtstar() {
        let dir = tempfile::tempdir().expect("temp dir creation");
        
        let fname = "testfile";
        
        let path = dir.path().join(fname);

        let _ = fs::File::create(&path).expect("file creation");

        let actual_lpath = ListedPath::from(path.clone());
        
        let correct_lpath = ListedPath::new(
            path,
            ListedPathType::File(
                ListedFileType::Other
            )
        );

        assert_eq!(actual_lpath, correct_lpath)
    }
    #[test]
    fn test_listed_path_from_pathbuf_on_dir() {
        let dir = tempfile::tempdir().expect("temp dir creation");
        
        let fname = "testdir";
        
        let path = dir.path().join(fname);

        fs::create_dir(&path).expect("dir creation");

        let actual_lpath = ListedPath::from(path.clone());
        
        let correct_lpath = ListedPath::new(
            path,
            ListedPathType::Dir
        );

        assert_eq!(actual_lpath, correct_lpath)
    }

    #[test]
    fn test_dirlister_walkdir0() {
        let root_dir = init_on_tempdir();
        let root = root_dir.path();
        let correct_list = correct_init_list(root);
        let actual_list: HashSet<ListedPath> = DirLister::walkdir(root)
            .expect("successful dirwalk")
            .into_iter()
            .collect();

        assert_eq!(actual_list, correct_list)
    }
}