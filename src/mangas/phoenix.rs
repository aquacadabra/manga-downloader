use std::vec::Vec;
use hyper::Client;
use hyper::header::Connection;
use std::io::Read;
use select::document::Document;
use select::predicate::*;

static HOME_PAGE_URL: &'static str = "http://www.nettruyen.com/truyen-tranh/vo-than-phuong-hoang";
pub static FOLDER_NAME: &'static str = "phoenix"; 

pub fn get_all_chapters_urls() -> Vec<String>{
	let mut vec: Vec<String>  = Vec::new();
	
	let client = Client::new();

	let mut resp;

	match client.get(HOME_PAGE_URL)
    	.header(Connection::close())
    	.send() {
		Err(_)=>{
			return vec;
		},        	
		Ok(r) => resp = r,
    }

    let mut page_content = String::new();    	
	match resp.read_to_string(&mut page_content){
		Err(_)=>{
			return vec;
		},
		_ => {},
	}	
	let document = Document::from_str(&page_content);  

	for node in document.find(Attr("class", "chapter")).find(Name("a")).iter() {                               
        match node.attr("href") {
            None=>{
                continue;
            },
            Some(href)=>{            	
                vec.push(href.to_string());                
            },
        }
    }
    vec.reverse();
	vec
}

pub fn get_all_image_urls_of_chapter(chapter_url: &str) -> Vec<String>{
	let mut vec: Vec<String>  = Vec::new();
	
	let client = Client::new();

	let mut resp;

	match client.get(chapter_url)
    	.header(Connection::close())
    	.send() {
		Err(_)=>{
			return vec;
		},        	
		Ok(r) => resp = r,
    }

    let mut page_content = String::new();    	
	match resp.read_to_string(&mut page_content){
		Err(_)=>{
			return vec;
		},
		_ => {},
	}	
	let document = Document::from_str(&page_content);  

	for node in document.find(Name("img")).iter() {                               
        match node.attr("data-index") {
            None=>{
                continue;
            },
            Some(_)=>{            	
                match node.attr("src"){
                	None=>{
                		continue;
                	},
                	Some(src)=>{
                		vec.push(src.to_string());
                	},
                }              
            },
        }
    }    
	vec
}