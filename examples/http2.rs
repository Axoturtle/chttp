fn main() {
    let response = chttp::http::Request::get("https://nghttp2.org")
        .extension(chttp::Options::default()
            .with_preferred_http_version(Some(chttp::http::Version::HTTP_2)))
        .body(())
        .map_err(Into::into)
        .and_then(chttp::send)
        .unwrap();

    println!("{:?}", response.headers());
}
