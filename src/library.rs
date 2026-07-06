#![allow(unused)]
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Debug)]
pub struct LibraryError;

#[derive(Debug)]
pub struct Library {
    albums: HashMap<PathBuf, Metadata>,
}

impl Library {
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, LibraryError> {
        let metadata_files = Self::find_metadata_files(root)?;

        let mut albums = HashMap::<PathBuf, Metadata>::new();
        for file in metadata_files {
            let album = crate::parser::parse_album(&file).unwrap();
            albums.insert(file, album);
        }

        Ok(Self { albums })
    }

    fn find_metadata_files<P: AsRef<Path>>(root: P) -> Result<Vec<PathBuf>, LibraryError> {
        let dir = std::fs::read_dir(root.as_ref()).unwrap();
        let metadata_files = glob::glob(&format!(
            "{}**/metadata.toml",
            &root.as_ref().to_str().unwrap()
        ))
        .unwrap()
        .filter_map(Result::ok)
        .collect();

        Ok(metadata_files)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    pub album: Album,
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Album {
    pub name: String,
    pub album_artist: String,
    pub date: String,
    pub original_date: String,
    pub genre: Vec<String>,
    pub labels: Vec<String>,
    pub media: String,
    pub disc_total: usize,
    pub track_total: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    pub title: String,
    pub track_number: usize,
    pub artists: Option<Vec<String>>,
    pub filename: Option<String>,
}
