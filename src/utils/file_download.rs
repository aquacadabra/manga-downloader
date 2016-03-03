use std::io;
use std::io::Error as IoError;
use std::path::Path;
use std::fs::File;
use hyper::error::Error as HyperError;
use hyper::Client;
use hyper::header::Connection;
use utils::file as myfileutil;

pub enum DownloadFileError {
    Network(HyperError),    
    Io(IoError),    
    None
}

// match utils::download_file("http://developer.highstreetvouchers.com/images/logo.png", "download"){
// 	DownloadFileError::Io(err) => println!("Io error {}", err),        
// 	DownloadFileError::Network(err) => println!("Network error {}", err),
// 	DownloadFileError::None => {},
// }

pub fn download_file(url: &str, folder_path: &str, file_name: &str)->DownloadFileError{
	let path = Path::new(folder_path);	
	//let file_name = myfileutil::get_file_name_from_url(url);	
	let path = path.join(file_name);
	let file_path = path.as_path().to_str().unwrap();
	//println!("{}", file_path);

	let mut file;
	match File::create(file_path) {
		Err(err)=>{
			return DownloadFileError::Io(err);
		},
		Ok(f) => file = f,
	}

	let client = Client::new();
	let mut res;
	match client.get(url)
    	.header(Connection::close())
    	.send() {
		Err(err)=>{
			return DownloadFileError::Network(err);
		},        	
		Ok(r) => res = r,
    }

    match io::copy(&mut res,&mut file){
    	Err(err)=>{
			return DownloadFileError::Io(err);
		},        	
		Ok(_) => {},
    }

	DownloadFileError::None
}