use askama::Template;
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
    pub gallery_opts: &'a GalleryOpts,

    pub image: &'a Image,
    pub previous_image: Option<&'a Image>,
    pub next_image: Option<&'a Image>,

    pub index: usize,
}