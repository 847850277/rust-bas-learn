pub(crate) async fn test() {
    let result =
        reqwest::get("https://github.com/DioxusLabs/awesome-dioxus/blob/1d03b42/awesome.json")
            .await;
    println!("{:?}", result);
}
