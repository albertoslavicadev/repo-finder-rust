use std::io;

fn main() {
    println!("Write the username");
    let mut _repo_name = String::new();

    io::stdin()
        .read_line(&mut _repo_name)
        .expect("Failed to read line");
        
    
    let url = format!("https://github.com/{}?tab=repositories", _repo_name.trim());

    let response = reqwest::blocking::get(
        url
        )
        .unwrap()
        .text()
        .unwrap();
        

        let document = scraper::Html::parse_document(&response);
        
        let repo_selector = scraper::Selector::parse("#user-repositories-list>ul>li").unwrap();

        let repos = document.select(&repo_selector).map(|x| x.inner_html());

        repos
            .zip(1..101)
            .for_each(|(item, number)| println!("{}. {}", number, item));

}

