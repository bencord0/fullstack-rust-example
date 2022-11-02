#[test]
fn test_client_js() {
    let client_js = client::get_file("client.js")
        .map(|f| f.contents_utf8())
        .flatten()
        .unwrap();

    assert!(client_js.contains("export function start() {"));
}
