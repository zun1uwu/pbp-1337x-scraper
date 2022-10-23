#![feature(option_result_contains)]

use scraper::{Html, Selector};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(r#"You must provide a search query, e.g. "php_1337x_scraper [query] [destination folder]""#);
        std::process::exit(0);
    }

    let query = &args[1];
    let dest_dir = &args[2];

    let cachestring = dest_dir.to_string();
    let cachedir = Path::new(&cachestring);
    let filename = format!(r"{}\1337x_Cache.json", cachedir.display());
    let filepath = Path::new(&filename);

    if filepath.exists() {
        println!(r"Clearing previous cache.");
        fs::remove_file(filepath).expect("Couldn't delete config file for re-caching");
    }
    
    let url = format!("https://1337x.to/category-search/{}/Games/1/", query);

    let body = reqwest::blocking::get(url)
        .expect("GET Request failed.")
        .text()
        .expect("Couldn't output HTML body as text.");

    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"tbody > tr > td > a"#).expect("Couldn't parse list");

    let mut results: Vec<String> = vec![];

    for title in document.select(&selector) {
        if title.html().contains("/torrent/") && results.len() < 6 {
            results.push(format!(
                "https://1337x.to{}",
                title.value().attr("href").unwrap()
            ));
        }
    }

    for entry in results {
        scan_page(entry, cachedir);
    }
}

fn scan_page(url: String, dest_dir: &Path) {
    let body = reqwest::blocking::get(url)
        .expect("GET Request failed.")
        .text()
        .expect("Couldn't output HTML body as text.");
    let document = Html::parse_document(&body);
    let title_selector = Selector::parse(r#"h1"#).expect("Couldn't parse title");
    let size_selector = Selector::parse(r#"ul > li > span"#).expect("Couldn't parse filesize");
    let magnet_selector = Selector::parse(r#"ul > li > a"#).expect("Couldn't parse magnets");

    let mut titles: Vec<String> = vec![];
    let mut sizes: Vec<String> = vec![];
    let mut magnets: Vec<String> = vec![];

    for title in document.select(&title_selector) {
        titles.push(title.inner_html());
    }

    for size in document.select(&size_selector) {
        if size.inner_html().contains("GB")
            || size.inner_html().contains("MB")
            || size.inner_html().contains("KB")
        {
            let html = size.inner_html();
            let split: Vec<&str> = html.split('<').into_iter().collect();
            let size = split[0];
            sizes.push(size.to_string());
        }
    }

    for magnet in document.select(&magnet_selector) {
        if magnet.inner_html().contains("Magnet Download") {
            magnets.push(
                magnet
                    .value()
                    .attr("href")
                    .expect("Couldn't push magnets to vector")
                    .to_string(),
            )
        }
    }

    let title = &titles[0];
    let size = &sizes[0];
    let magnet = &magnets[0];

    write_to_json(title.to_string(), size.to_string(), magnet.to_string(), dest_dir);
}

fn write_to_json(title: String, size: String, magnet: String, dest_dir: &Path) {
    println!("Caching: {}", title);
    let jsoncontent = format!(
        r#"{{ "title": "{}", "size": "{}", "download": "{}" }}
"#,
        title, size, magnet
    );

    let dir_string = dest_dir;
    let dir_path = Path::new(&dir_string);
    let file_string = format!(r"{}\1337x_Cache.json", dir_path.display());
    let file_path = Path::new(&file_string);

    if !dir_path.exists() {
        fs::create_dir_all(dir_path).expect("Couldn't create config directory");
    }

    if !file_path.exists() {
        File::create(file_path).expect("Couldn't create config file");
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .expect("Opening file failed");

    file.write_all(jsoncontent.as_bytes())
        .expect("Couldn't write bytes to file");
}
