use askama::Template;
use rust_embed::Embed;
use crate::gallery::{Gallery, GalleryOpts};
use crate::image::Image;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub gallery: &'a Gallery,
    pub gallery_opts: &'a GalleryOpts,
    // pub title: &'a str,
}


#[derive(Template)]
#[template(path = "image.html")]
pub struct ImageTemplate<'a> {
    pub gallery: &'a Gallery,

    pub image: &'a Image,
    pub previous_image: Option<&'a Image>,
    pub next_image: Option<&'a Image>,

    pub title: Option<String>,
    pub place: Option<String>,
    pub author: Option<String>,
    pub index: usize,
}


#[derive(Embed)]
#[folder = "static/"]
pub struct StaticFiles;