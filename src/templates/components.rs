use askama::Template;

#[derive(Template)]
#[template(path = "components/about.html")]
pub struct About {}

#[derive(Template)]
#[template(path = "components/dev.html")]
pub struct Dev {}

#[derive(Debug)]
pub struct ArtCollection {
    pub name: String,
    pub links: Vec<String>,
}

#[derive(Template)]
#[template(path = "components/art.html")]
pub struct Art<'a> {
    pub artworks: &'a [ArtCollection],
}

#[derive(Template)]
#[template(path = "components/artwork.html")]
pub struct ArtWork<'a> {
    pub art: &'a ArtCollection,
}

#[derive(Template)]
#[template(path = "components/thumbnail.html")]
pub struct Thumbnail<'a> {
    collection: &'a ArtCollection,
}

#[derive(Template)]
#[template(path = "components/exp.html")]
pub struct Exp {}

#[derive(Template)]
#[template(path = "components/carousel.html")]
pub struct Carousel<'a> {
    pub collection: &'a ArtCollection,
    pub current: usize,
}
