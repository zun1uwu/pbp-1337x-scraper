#![windows_subsystem = "windows"]
#![feature(option_result_contains)]

use std::{env, vec, fs::{self, File}, path::Path, io::Write};
use scraper::{Html, Selector};

fn main() {
    
    let query: Vec<String> = env::args().collect();

    if query.len() < 2 {
        println!(r#"You must provide a search query, e.g. "php_1337x_scraper 'gta'""#);
        std::process::exit(0);
    }

    let query = &query[1];
    let url = format!("https://1337x.to/category-search/{}/Games/1/", query);

    let body = reqwest::blocking::get(url).expect("GET Request failed.").text().unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse(r#"tbody > tr > td > a"#).unwrap();

    let mut results: Vec<String> = vec![];

    for title in document.select(&selector) {
        if title.html().contains("/torrent/") && results.len() < 6 {
            results.push(format!("https://1337x.to{}", title.value().attr("href").unwrap().to_string()));
        }
    }

    for entry in results {
        scan_page(entry);
    }
}

fn scan_page(url: String) {

    let body = reqwest::blocking::get(url).expect("GET Request failed.").text().unwrap();
    let document = Html::parse_document(&body);
    let title_selector = Selector::parse(r#"h1"#).unwrap();
    let size_selector = Selector::parse(r#"ul > li > span"#).unwrap();
    let magnet_selector = Selector::parse(r#"ul > li > a"#).unwrap();

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
            let split: Vec<&str> = html.split("<").into_iter().collect();
            let size = split[0];
            sizes.push(size.to_string());
        }
    }

    for magnet in document.select(&magnet_selector) {
        if magnet.inner_html().contains("Magnet Download") {
            magnets.push(magnet.value().attr("href").unwrap().to_string())
        }
    }

    let title = &titles[0];
    let size = &sizes[0];
    let magnet = &magnets[0];

    write_to_json(title.to_string(), size.to_string(), magnet.to_string());

}

fn write_to_json(title: String, size: String, magnet: String) {

    let jsoncontent = format!(
        r#"{{ "title": "{}", "size": {}, "magnet": "{}" }}
"#,
        title, size, magnet
    );

    let dir_string = format!(r"C:\Users\{}\AppData\Roaming\Project Black Pearl\", whoami::username());
    let dir_path = Path::new(&dir_string);
    let file_string = format!(r"{}\Cache.json", dir_path.display().to_string());
    let filepath = Path::new(&file_string);

    if !dir_path.exists() {
        fs::create_dir_all(dir_path).unwrap();
    }
    
    if !filepath.exists() {
        File::create(filepath).unwrap();
    }

    let mut file = fs::OpenOptions::new().write(true).append(true).open(filepath).unwrap();
    file.write_all(jsoncontent.as_bytes()).unwrap();
}