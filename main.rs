fn main() {
    let url = "https://www.genie.co.kr/chart/top200";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().unwrap();
    
    use scraper::{Html, Selector};
    let document = Html::parse_document(&body);
    let song_selector = Selector::parse("td.info").unwrap();
    let title_selector = Selector::parse("a.title").unwrap();
    let artist_selector = Selector::parse("a.artist").unwrap();

    let mut top_t: Vec<String> = Vec::new();
    let mut top_a: Vec<String> = Vec::new();

    for element in document.select(&song_selector) {
        let song_name_element = element.select(&title_selector).next().expect("Could not select book name.");
        let song_name = song_name_element.text().collect::<String>();
        let title = song_name.replace("\n", "").replace("  ", "");
        top_t.push(title);

        let artist_name_element = element.select(&artist_selector).next().expect("Could not select book name.");
        let artist_name = artist_name_element.text().collect::<String>();
        let artist = artist_name.replace("\n", "").replace("  ", "");
        top_a.push(artist);
    }

    let dt_selector = Selector::parse("div.date").unwrap();
    let date_selector = Selector::parse("h3 span").unwrap();
    let time_selector = Selector::parse("h3 time span").unwrap();
    for element in document.select(&dt_selector) {
        let date_element = element.select(&date_selector).next().expect("Could not select book name.");
        let date_raw = date_element.text().collect::<String>();
        println!("{:?}", date_raw);
        let time_element = element.select(&time_selector).next().expect("Could not select book name.");
        let time_raw = time_element.text().collect::<String>();
        println!("{:?}", time_raw);
    }

    let  mut i = 0 ;
    while i < top_t.len() {
        println!("{:?} {:?} - {:?}", i+1, top_t[i], top_a[i]);
        i+=1;
    }
}