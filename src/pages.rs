use askama::Template;
use crate::gallery::{Gallery, GalleryOpts};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub gallery: &'a Gallery,
    pub gallery_opts: &'a GalleryOpts,
    // pub title: &'a str,
}
