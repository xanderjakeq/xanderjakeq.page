use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Error;

use human_sort::sort;
use salvo::handler;
use salvo::Depot;

use crate::templates::ArtCollection;

#[handler]
fn check_hx() {}

pub fn get_art_images() -> Result<Vec<ArtCollection>, Error> {
    let mut art_collection: Vec<ArtCollection> = vec![];

    for entry in read_dir("./static/images/art")? {
        let dir_entry = entry?;

        if let Some(path_str) = dir_entry.path().to_str() {
            if !&path_str[1..].contains(".") {
                let mut links: Vec<String> = vec![];

                for art_entry in read_dir(path_str)? {
                    let art_dir_entry = art_entry?;

                    if let Some(path_str) = art_dir_entry.path().to_str() {
                        if path_str.ends_with(".webp") {
                            continue;
                        };
                        links.push(path_str[1..].to_owned());
                    }
                }

                let mut str_links = links.iter().map(|l| l.as_ref()).collect::<Vec<&str>>();

                sort(&mut str_links);

                let collection = ArtCollection {
                    name: path_str[1..].split("/").last().unwrap().to_string(),
                    links: str_links
                        .iter()
                        .map(|l| l.to_string())
                        .collect::<Vec<String>>(),
                };

                art_collection.push(collection);
            }
        }
    }

    return Ok(art_collection);
}

#[handler]
pub fn init_data(depot: &mut Depot) {
    let artworks = get_art_images().unwrap();
    let mut art_map: HashMap<String, ArtCollection> = HashMap::new();

    for art in artworks {
        art_map.insert(art.name.to_string(), art);
    }

    depot.insert("artwork_map", art_map);
}
