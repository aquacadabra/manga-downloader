use std::io;
use std::io::Error as IoError;
use std::path::Path;
use std::fs::File;
use hyper::error::Error as HyperError;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::ContentType;

pub enum DownloadFileError {
    Network(HyperError),    
    Io(IoError),    
    WrongContentType,
    None
}

// match utils::download_file("http://developer.highstreetvouchers.com/images/logo.png", "download"){
// 	DownloadFileError::Io(err) => println!("Io error {}", err),        
// 	DownloadFileError::Network(err) => println!("Network error {}", err),
// 	DownloadFileError::None => {},
// }

pub fn download_image_file(url: &str, folder_path: &str, file_name: &str)->DownloadFileError{
	let file_name_string = file_name.to_string();
	if !(file_name_string.ends_with(".png") ||
			file_name_string.ends_with(".jpg")){
		return DownloadFileError::WrongContentType;	
	}

	let path = Path::new(folder_path);	
	//let file_name = myfileutil::get_file_name_from_url(url);	
	let path = path.join(file_name);
	let file_path = path.as_path().to_str().unwrap();
	//println!("{}", file_path);

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
    
    let headers = res.headers.clone();
    let header;
    match headers.get::<ContentType>(){
    	Some(h)=>{
    		header = h;
    	},
    	None=>{
    		return DownloadFileError::WrongContentType;	
    	},
    }
    if !header.to_string().starts_with("image/"){
    	return DownloadFileError::WrongContentType;	
    }

    let mut file;
	match File::create(file_path) {
		Err(err)=>{
			return DownloadFileError::Io(err);
		},
		Ok(f) => file = f,
	}	

    match io::copy(&mut res,&mut file){
    	Err(err)=>{
			return DownloadFileError::Io(err);
		},        	
		Ok(_) => {},
    }

	DownloadFileError::None
}