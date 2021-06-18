# Rustium
Medium feed as Json built with Rust.
## How it works?
`rustium-api` makes requests to `https://medium.com/AUTHOR/feed`, receiving RSS as `xml` and converting to Json.
### Medium RSS Feed (example)
```xml
<?xml version="1.0" encoding="UTF-8"?><rss xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:atom="http://www.w3.org/2005/Atom" version="2.0" xmlns:cc="http://cyber.law.harvard.edu/rss/creativeCommonsRssModule.html">
    <channel>
        <title><![CDATA[Stories by Jeziel Lago on Medium]]></title>
        <description><![CDATA[Stories by Jeziel Lago on Medium]]></description>
        <link>https://medium.com/@jeziellago?source=rss-e3c73726655c------2</link>
        <image>
            <url>https://cdn-images-1.medium.com/fit/c/150/150/1*wU_0x6MywryLcBgBPD2GAQ.jpeg</url>
            <title>Stories by Jeziel Lago on Medium</title>
            <link>https://medium.com/@jeziellago?source=rss-e3c73726655c------2</link>
        </image>
        <generator>Medium</generator>
        ...
        <item>
            <title><![CDATA[Title]]></title>
            <link>https://example.medium.com/post</link>
            <guid isPermaLink="false">https://medium.com/p/4ec4041fa260</guid>
            <category><![CDATA[concurrent-programming]]></category>
            <category><![CDATA[software-development]]></category>
            <category><![CDATA[threads]]></category>
            <category><![CDATA[actor-model]]></category>
            <category><![CDATA[actors]]></category>
            <content:encoded><![CDATA[POST HTML CONTENT]]></content:encoded>
        </item>
        <item>...</item>
        <item>...</item>
        <item>...</item>
    </channel>
</rss>
</xml>
```
### Rustium enable Feed as Json
```json
{
    "posts": [
        {
            "title": "Something about anything",
            "link": "https://example.medium.com/some-post-title",
            "category": ["tech", "rust", "api"],
            "content": "<html post content here>"
        },
        ...
    ]
}
```
## Request
```text
GET
http://localhost:3030/posts/AUTHOR
```
## Run
```
$ cd rustium-sample
$ cargo run
```
