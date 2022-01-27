use image::io::Reader as ImageReader;

use std::fs::File;
use std::io::Write;

///Converts image to BHI format.
pub fn img_bhi(file_path:&String, new_name:&String) {

    let decoded = match ImageReader::open(file_path) {
        Err(err) => {
            println!("Error: Image file at '{}' not found. {}", file_path, err);
            return;
        },
        Ok(res) => match res.with_guessed_format().unwrap().decode() {
            Err(err) => {
                println!("Error: Image file malformed at '{}'. {}", file_path, err);
                return;
            },
            Ok(r) => r
        }
    };

    let rgb_decode = decoded.to_rgb8();
    let dim = rgb_decode.dimensions();
    let mut buffer = format!("{}x{}:", dim.0, dim.1); 



    for i in rgb_decode.pixels() {
        
        let fmt = format!("{}.{}.{},", i[0], i[1], i[2]);
        buffer.push_str(fmt.as_str());

    }


    let mut file = match File::create(format!("{}.bhi", new_name)) {
        Err(_) => panic!("Cannot create file."),
        Ok(file) => file 
    };


    let raw = buffer.into_bytes();

    let mut compressed = snap::write::FrameEncoder::new(vec![]);

    compressed.write_all(&raw);
    let res = compressed.into_inner().unwrap();
    
    file.write_all(&res);

}



pub fn convert_to_BHI(file_path: String, new_name: String) {

    

    if file_path.ends_with(".jpg") || file_path.ends_with(".png") || file_path.ends_with(".ico") || file_path.ends_with(".bmp") || file_path.ends_with(".tiff") {

        img_bhi(&file_path, &new_name);

    } else if file_path.ends_with(".bhi") {



    } else {

        println!("Error: Unsupported file type. File types supported are; JPEG, PNG, ICO, BMP, and TIFF.");
        return;

    }
}