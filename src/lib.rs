use serde::Serialize;
use serde_json::Value;
use slug::slugify;
use std::collections::{BTreeMap, BTreeSet};

#[derive(PartialEq, Serialize, Debug)]
pub struct BlogPost {
    frontmatter: BTreeMap<String, serde_json::Value>,
    title: String,
    slug: String,
    tags: BTreeSet<String>,
    body: String,
}

impl BlogPost {
    pub fn new(title: String) -> Template {
        Template {
            slug: slugify(&title),
            title: title,
            meta: BTreeMap::new(),
            tags: BTreeSet::new(),
            body: None,
        }
    }
    pub fn as_frontmatter(&self) -> String {
        let yaml = serde_yaml::to_string(&self.frontmatter)
            .unwrap();
        yaml
    }
    pub fn as_file(&self) -> String {
        format!(
            "{}---\n\n{}",
            &self.as_frontmatter(),
            self.body
        )
    }
}

impl From<Template> for BlogPost {
    fn from(template: Template) -> Self {
        let mut frontmatter = template.meta;
        frontmatter.insert(
            "slug".to_string(),
            Value::String(template.slug.clone()),
        );
        frontmatter.insert(
            "title".to_string(),
            serde_json::Value::String(
                template.title.clone(),
            ),
        );
        frontmatter.insert(
            "tags".to_string(),
            template
                .tags
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .into(),
        );

        BlogPost {
            frontmatter,
            title: template.title,
            tags: template.tags,
            slug: template.slug,
            body: template.body.unwrap_or("".to_string()),
        }
    }
}

// BlogPostBuilder
pub struct Template {
    title: String,
    slug: String,
    tags: BTreeSet<String>,
    meta: BTreeMap<String, serde_json::Value>,
    body: Option<String>,
}

impl Template {
    pub fn slug(&mut self, slug: &str) -> &mut Template {
        self.slug = slugify(slug);
        self
    }
    pub fn tag(&mut self, tag: &str) -> &mut Template {
        self.tags.insert(slugify(&tag));
        self
    }
    pub fn property(
        &mut self,
        key: String,
        value: serde_json::Value,
    ) -> &mut Template {
        self.meta.insert(key, value);
        self
    }
    pub fn post(mut self, blog_post: String) -> BlogPost {
        self.body = Some(blog_post);
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let post = BlogPost::new(
            "Builder APIs in Rust".to_string(),
        )
        .post("do this, then that".to_string());

        assert_eq!(
            BlogPost {
                title: "Builder APIs in Rust".to_string(),
                frontmatter: BTreeMap::from([
                    (
                        "slug".to_string(),
                        Value::String(
                            "builder-apis-in-rust"
                                .to_string()
                        ),
                    ),
                    (
                        "tags".to_string(),
                        Value::Array(vec![])
                    ),
                    (
                        "title".to_string(),
                        Value::String(
                            "Builder APIs in Rust"
                                .to_string()
                        )
                    )
                ]),
                tags: BTreeSet::new(),
                slug: "builder-apis-in-rust".to_string(),
                body: "do this, then that".to_string()
            },
            post
        );
    }

    #[test]
    fn to_frontmatter() {
        let post = BlogPost::new(
            "Builder APIs in Rust".to_string(),
        )
        .post("do this, then that".to_string());

        assert_eq!(
            "slug: builder-apis-in-rust
tags: []
title: Builder APIs in Rust
",
            post.as_frontmatter()
        );
    }

    #[test]
    fn to_file() {
        let post = BlogPost::new(
            "Builder APIs in Rust".to_string(),
        )
        .post("do this, then that".to_string());

        assert_eq!(
            "slug: builder-apis-in-rust
tags: []
title: Builder APIs in Rust
---

do this, then that",
            post.as_file()
        );
    }
}
