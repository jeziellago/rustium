#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate serde_xml_rs;

pub mod rustium {

    const MEDIUM_BASE_URL: &str = "https://medium.com/";

    use hyper::client::connect::HttpConnector;
    use hyper::{Body, Client, Response, Uri};
    use hyper_tls::HttpsConnector;
    use std::io::{Error, ErrorKind};
    use tokio_compat_02::FutureExt;

    pub use crate::mediumrss;

    #[derive(Debug, Serialize)]
    pub struct RSSFeed {
        pub posts: Vec<Post>,
    }

    #[derive(Debug, Serialize)]
    pub struct Post {
        pub title: String,
        pub link: String,
        pub category: Vec<String>,
        pub content: String,
    }

    pub async fn get_posts(
        http_client: &Client<HttpsConnector<HttpConnector>, Body>,
        creator: String,
    ) -> Result<RSSFeed, std::io::Error> {
        let feed_url: Uri = format!("{}@{}/feed", MEDIUM_BASE_URL, creator)
            .parse()
            .unwrap();

        let response: Result<Response<Body>, hyper::Error> =
            http_client.get(feed_url).compat().await;

        let body = match response {
            Ok(response_body) => hyper::body::aggregate(response_body).compat().await,
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };

        let xml_feed = match body {
            Ok(content) => mediumrss::parse_as_feed(content),
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };

        return match xml_feed {
            Ok(feed) => Ok(map_to_rssfeed(feed.channel.posts)),
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };
    }

    fn map_to_rssfeed(xml_posts: Vec<mediumrss::XmlPost>) -> RSSFeed {
        let posts: Vec<Post> = xml_posts
            .iter()
            .map(|xml_post: &mediumrss::XmlPost| Post {
                title: String::from(xml_post.title.as_str()),
                link: String::from(xml_post.link.as_str()),
                category: xml_post
                    .category
                    .iter()
                    .map(|c: &String| String::from(c.as_str()))
                    .collect(),
                content: String::from(xml_post.content.as_str()),
            })
            .collect();
        return RSSFeed { posts: posts };
    }

    #[cfg(test)]
    mod tests {

        pub use crate::mediumrss;
        pub use crate::rustium;

        #[test]
        fn validate_parse_to_rssfeed() {
            let mut category: Vec<String> = Vec::new();
            category.push(String::from("tech"));
            category.push(String::from("rust"));

            let mut xml_posts: Vec<mediumrss::XmlPost> = Vec::new();

            xml_posts.push(mediumrss::XmlPost {
                title: String::from("title"),
                link: String::from("link"),
                content: String::from("content"),
                category: category.clone(),
            });

            let rssfeed: rustium::RSSFeed = rustium::map_to_rssfeed(xml_posts);
            assert_eq!("title", rssfeed.posts[0].title);
            assert_eq!("link", rssfeed.posts[0].link);
            assert_eq!("content", rssfeed.posts[0].content);
            assert_eq!(category, rssfeed.posts[0].category);
        }
    }
}

pub mod mediumrss {

    #[derive(Debug, Deserialize)]
    #[serde(rename = "rss")]
    pub struct XmlFeed {
        #[serde(rename = "channel")]
        pub channel: XmlChannel,
    }

    #[derive(Debug, Deserialize)]
    pub struct XmlChannel {
        #[serde(rename = "item", default)]
        pub posts: Vec<XmlPost>,
    }

    #[derive(Debug, Deserialize)]
    pub struct XmlPost {
        pub title: String,
        pub link: String,
        pub category: Vec<String>,
        #[serde(rename = "encoded", default)]
        pub content: String,
    }

    pub fn parse_as_feed(content: impl hyper::body::Buf) -> Result<XmlFeed, serde_xml_rs::Error> {
        serde_xml_rs::from_reader(content.reader())
    }
}
