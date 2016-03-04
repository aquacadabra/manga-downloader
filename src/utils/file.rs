pub fn get_file_name_from_url(url: &str)->&str{
	let mut tokens = url.split("?");
	let uri = tokens.nth(0).unwrap();
	tokens = uri.split("/");	
	return tokens.last().unwrap();
}

pub fn get_file_extension_from_filename(filename: &str)->&str{
	let tokens = filename.split(".");
	return tokens.last().unwrap();
}