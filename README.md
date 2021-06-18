# Rustium
Medium feed as Json built with Rust.
## How it works?
`rustium-api` makes requests to `https://medium.com/AUTHOR/feed`, receiving RSS as `xml` and converting to Json.
## Request
```text
GET
http://localhost:3030/posts/AUTHOR
```
## Response
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
## Run
```
$ cd rustium-sample
$ cargo run
```
