extern crate hyper;
extern crate select;

mod utils;
mod mangas;

use std::fs;
use std::path::Path;
use utils::DownloadFileError;
use mangas::phoenix as manga;

fn main() {		
    let chapters_urls = manga::get_all_chapters_urls();
    let mut chap_count: i32 = 1;
    for chap in chapters_urls{    	
    	println!("Process chapter: {}", chap_count);
    	process_chapter(&chap, chap_count);
    	chap_count = chap_count + 1;    	
    }
    println!("{:04}", chap_count);
}

fn process_chapter(url: &str, chap_number: i32){
	let path = Path::new("download");
	let path = path.join(manga::FOLDER_NAME);
	let path = path.join(format!("{:04}",chap_number));
	fs::create_dir_all(path.as_path());	
	let image_urls = manga::get_all_image_urls_of_chapter(url);
	let mut img_count: i32 = 1;
	for img in image_urls{		
		let img_name = utils::get_file_name_from_url(&img);		
		let img_extension = utils::get_file_extension_from_filename(img_name);
		let new_img_name = format!("{:04}", img_count).to_string() + ".";
		let new_img_name = new_img_name + img_extension;
		let new_img_name = new_img_name.to_lowercase();
		match utils::download_image_file(&img, path.as_path().to_str().unwrap(), &new_img_name){
			DownloadFileError::None =>{
				img_count = img_count + 1;
			},
			_ => {},
		}
		
	}
}
