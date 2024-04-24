use serde::{Deserialize, Serialize};

trait FeedElement {
    fn cleanup_authors(&mut self);

    fn cleanup_authors_impl(author: &mut Option<Author>, authors: &mut Option<Vec<Author>>) {
        if author.is_some() {
            if authors.is_none() {
                *authors = Some(vec![author.as_ref().unwrap().clone()]);
            }

            *author = None;
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    pub name: Option<String>,
    pub url: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hub {
    pub r#type: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachment {
    pub url: String,
    pub mime_type: String,
    pub title: Option<String>,
    pub size_in_bytes: Option<u64>,
    pub duration_in_seconds: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub url: Option<String>,
    pub external_url: Option<String>,
    pub title: Option<String>,
    pub content_html: Option<String>,
    pub content_text: Option<String>,
    pub summary: Option<String>,
    pub image: Option<String>,
    pub banner_image: Option<String>,
    pub date_published: Option<String>,
    pub date_modified: Option<String>,
    pub authors: Option<Vec<Author>>,
    author: Option<Author>, // for compatibility with JSON Feed 1.0
    pub tags: Option<Vec<String>>,
    pub language: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
}

impl FeedElement for Item {
    fn cleanup_authors(&mut self) {
        <Self as FeedElement>::cleanup_authors_impl(&mut self.author, &mut self.authors);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    pub version: String,
    pub title: String,
    pub home_page_url: Option<String>,
    pub feed_url: Option<String>,
    pub description: Option<String>,
    pub user_comment: Option<String>,
    pub next_url: Option<String>,
    pub icon: Option<String>,
    pub favicon: Option<String>,
    pub authors: Option<Vec<Author>>,
    author: Option<Author>, // for compatibility with JSON Feed 1.0
    pub language: Option<String>,
    pub expired: Option<bool>,
    pub hubs: Option<Vec<Hub>>, // TODO: Can this be used in output?
    pub items: Option<Vec<Item>>,
}

impl FeedElement for Feed {
    fn cleanup_authors(&mut self) {
        <Self as FeedElement>::cleanup_authors_impl(&mut self.author, &mut self.authors);
    }
}

impl Feed {
    pub fn parse(data: &str) -> Result<Self, serde_json::Error> {
        let mut feed = serde_json::from_str::<Feed>(data)?;
        feed.cleanup_authors();

        if let Some(ref mut items) = feed.items {
            for item in items.iter_mut() {
                item.cleanup_authors();
            }
        }

        Ok(feed)
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
