use image::io::Reader as ImageReader;

use std::fs::File;
use std::io::Write;



//This file is purposed for the encoder that converts to the BHI format, from other existing file formats.


/**
 * Converts to BHI format from supported image formats.
 * @file_path Path to subject file.
 * @new_name Name of output file.
 */
pub fn img_bhi(file_path:&String, new_name:&String) {

    //Read image. Catch unfound or bad file.

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

    let rgb_decode = decoded.to_rgb8();                 //Decode image.
    let dim = rgb_decode.dimensions();                  //Retrieve dimensions.
    let mut buffer = format!("{}x{}:", dim.0, dim.1);   //Formatted meta data for dimensions of image file.


    //Format is as follows; XDimension(x)YDimension: R.G.B,...

    for i in rgb_decode.pixels() {
        
        //Format for each RGB value.
        let fmt = format!("{}.{}.{},", i[0], i[1], i[2]);
        buffer.push_str(fmt.as_str());

    }


    //Create out-file.
    let mut file = match File::create(format!("{}.bhi", new_name)) {
        Err(_) => panic!("Cannot create file."),
        Ok(file) => file 
    };


    //Ready format for compression.
    let raw = buffer.into_bytes();
    let mut compressed = snap::write::FrameEncoder::new(vec![]);

    //Write to compression stream.
    compressed.write_all(&raw);
    let res = compressed.into_inner().unwrap();
    
    //Final write.
    file.write_all(&res);
}


/**
 * Top-level interface function for converting from supported image types; to BHI format.
 * @file_path Path to image.
 * @new_name Out-file name.
 */
pub fn convert_to_BHI(file_path: String, new_name: String) {

    
    //Supported image formats.

    if file_path.ends_with(".jpg") || file_path.ends_with(".png") || file_path.ends_with(".ico") || file_path.ends_with(".bmp") || file_path.ends_with(".tiff") {

        img_bhi(&file_path, &new_name);

    } else {
        println!("Error: Unsupported file type. File types supported are; JPEG, PNG, ICO, BMP, and TIFF.");
        return;
    }
}