use std::collections::HashMap;

use askama::Template;
use salvo::http::header::{self, HeaderValue};
use salvo::{handler, http::StatusCode, writing::Text, Depot, Request, Response};

use crate::handlers::get_art_images;
use crate::templates::{About, Art, ArtCollection, ArtWork, Dev, Exp, Index};

#[handler]
pub fn render_index(req: &mut Request, res: &mut Response) {
    let about = About {};
    let index = Index {
        section: about.render().unwrap(),
    };

    if let Ok(html_string) = index.render() {
        res.render(Text::Html(html_string));
    } else {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
}

#[handler]
pub fn render_section(req: &mut Request, res: &mut Response) {
    let is_hx = req.headers().get("HX-Request").is_some();

    if let Some(section) = req.params().get("section") {
        let section_render = match &section[..] {
            "about" => About {}.render(),
            "dev" => Dev {}.render(),
            "art" => {
                let artworks = get_art_images().unwrap();

                Art {
                    artworks: &artworks,
                }
                .render()
            }
            _ => {
                res.status_code(StatusCode::NOT_FOUND);
                return;
            }
        };

        if let Ok(html_string) = section_render {
            if !is_hx {
                let index = Index {
                    section: html_string,
                };

                if let Ok(index_html_string) = index.render() {
                    res.render(Text::Html(index_html_string));
                }

                return;
            }

            let headers = res.headers_mut();
            headers.insert("Cache-Control", HeaderValue::from_static("max-age=604800"));

            res.render(Text::Html(html_string));
            return;
        } else {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    }

    res.status_code(StatusCode::NOT_FOUND);
    return;
}

#[handler]
pub fn render_artwork(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let is_hx = req.headers().get("HX-Request").is_some();

    if let Some(artwork) = req.params().get("artwork") {
        let art_map = depot
            .get::<HashMap<String, ArtCollection>>("artwork_map")
            .unwrap();

        let artwork_render = ArtWork {
            art: art_map.get(artwork).unwrap(),
        }
        .render();

        let headers = res.headers_mut();
        headers.insert("Cache-Control", HeaderValue::from_static("max-age=604800"));
        headers.insert("Vary", HeaderValue::from_static("HX-Request"));

        if let Ok(artwork_html_string) = artwork_render {
            if !is_hx {
                let index = Index {
                    section: artwork_html_string.to_string(),
                };

                if let Ok(index_html_string) = index.render() {
                    res.render(Text::Html(index_html_string));
                }
                return;
            }

            res.render(Text::Html(artwork_html_string));
        }
    }
}
