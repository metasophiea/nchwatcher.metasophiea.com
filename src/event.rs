use regex::Regex;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};
use time::{Date, Time};
use time::macros::format_description;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    url: String,
    #[serde(default)]
    image_url: Option<String>,
    date: Date,
    #[serde(default)]
    time: Option<Time>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    meta: Vec<String>
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.date == other.date
    }
}

impl Event {
    pub fn get_image_url(&self) -> &Option<String> { &self.image_url }
    pub fn get_date(&self) -> &Date { &self.date }
    pub fn get_time(&self) -> &Option<Time> { &self.time }

    pub fn set_image_url(&mut self, url:String) { self.image_url = Some(url) }
}

impl Event {
    pub fn extract_events_from_html_document(document:&Html) -> Vec<Event> {
        let regex = Regex::new(r"\d{4}").unwrap();

        document.select(&Selector::parse(r#"div[class="feature-card"]"#).unwrap())
            .map(|element| {
                let mut url = String::new();
                let mut image_url = None;
                let mut date_and_time_string = None;
                let mut title = None;
                let mut meta = vec![];

                element.child_elements().for_each(|element| {
                    if element.attr("class") == Some("card-img") {
                        url = element.attr("href").unwrap().to_string();
                        element.select( &Selector::parse(r#"img"#).unwrap() ).for_each(|element| {
                            image_url = Some(element.attr("src").unwrap().to_string());
                        });
                    }

                    if element.attr("class") == Some("card-body") {
                        element.select( &Selector::parse(r#"a[class="title"]"#).unwrap() ).for_each(|element| {
                            title = Some(element.inner_html());
                        });
                        element.select( &Selector::parse(r#"p[class="meta"]"#).unwrap() ).for_each(|element| {
                            let mut string = element.inner_html();
                            if regex.is_match(&string) {
                                string = string.replace(" (MULTIPLE PERFORMANCES)", "");
                                string = string.split(" to ").next().unwrap().to_string();
                                date_and_time_string = Some(string);
                            } else {
                                meta.push(string);
                            }
                        });
                    }
                });


                let format = format_description!(version = 2, "[weekday] [day padding:none] [month padding:none repr:short] [year][optional [ ]][optional [[hour padding:none repr:12]:[minute][period case:upper]]]");
                let date = Date::parse(date_and_time_string.as_ref().unwrap(), &format)
                    .unwrap_or_else(
                        |err| panic!("ERROR - Event::extract_events_from_html_document - could not parse date - err:{err} date_and_time_string:{date_and_time_string:?}")
                    );
                let time = Time::parse(date_and_time_string.as_ref().unwrap(), &format).ok();

                Event {
                    url,
                    image_url,
                    date,
                    time,
                    title,
                    meta
                }
            })
            .collect()
    }
}