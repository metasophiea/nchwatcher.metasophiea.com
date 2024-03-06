use curl::easy::Easy;
use scraper::{Html, Selector};

use super::constants::{
    ALL_EVENTS_PAGE_URL_TAIL,
    BASE_URL,
    MAX_PAGE_NUMBER
};

pub fn request_url(url:&str) -> String {
    let mut buffer = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();

    let mut transfer = handle.transfer();
    transfer.write_function(|data| {
        buffer.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    let output = std::str::from_utf8(&buffer).unwrap().to_string();
    output
}

pub fn gather_all_pages() -> Vec<Html> {
    let mut output = vec![];

    let mut is_last_page = false;
    let mut count = 0;
    while !is_last_page {
        count += 1;
        if count >= MAX_PAGE_NUMBER {
            panic!("ERROR - gather_all_pages - exceeded max page number");
        }

        let url = format!("{BASE_URL}/{ALL_EVENTS_PAGE_URL_TAIL}/?page={count}");
        println!(">> {url}");
        let html_string = request_url(&url);
        let document = Html::parse_document(&html_string);

        //check if this is the last page
        is_last_page = document.select(&Selector::parse(r#"nav[aria-label="Pagination"]"#).unwrap()).any(|element|{
            let element = element.select(&Selector::parse(r#"ul"#).unwrap()).next().unwrap();
            let element = element.select(&Selector::parse(r#"li"#).unwrap()).last().unwrap();
            let element = element.child_elements().next().unwrap();
            element.attr("class") == Some("is-disabled") && element.inner_html() == "Next"
        });

        output.push(document);
    }

    output
}
