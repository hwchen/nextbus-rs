// So, the plan is not to look at particular stops; but to look at
// when the bus passes a certain latitude (78) or longitude (74).
// This means i don't really need route info. Just bus info.
//
// Poll every 10 seconds
extern crate hyper;
extern crate kuchiki;

// use std::io::Read;

use hyper::Client;
use kuchiki::Html;

fn main() {
    // nextbus tag is "MBTA"
    let url =
        "http://webservices.nextbus.com/service/publicXMLFeed?command=vehicleLocations&a=mbta&r=74&t=0";

    let client = Client::new();

    let mut res = client.get(url)
        .send()
        .unwrap();

//    let mut body = String::new();
//    res.read_to_string(&mut body).unwrap();
//    println!("Response {}", body);

    if let Ok(xml) = Html::from_stream(&mut res) {
        let doc = xml.parse();
//        let body = doc.children()
//            .nth(1).expect("Fail on first parse").children() //returns inside <html>
//            .nth(1).expect("Fail parse body"); //returns body

//        let comment = body.to_string();
//        println!("Comment: {}", comment);

        println!("All: {}", doc.to_string());


    } else {
        println!("{}", "page couldn't be parsed");
    }
}
