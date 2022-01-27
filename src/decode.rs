
use image::{RgbImage, Rgb};

use std::fs::File;
use std::io::Read;

//The purpose of this file is to convert BHI files to other supported file types.

/**
 * Get dimensions of raw BHI file based on expanded buffer.
 * @buffer Expanded, raw, BHI file
 * @returns (X, Y) tuple 
 */
pub fn get_dimensions(buffer: &String) -> (u32, u32) {
    let mut dimensions:String = String::new();
    
    for i in buffer.chars() { 
        if i == ':' {
            break;
        } else {
            dimensions.push(i);
        }
    }

    let spl: Vec<&str> = dimensions.split("x").collect();

    //Cast.

    let xdim: u32 = spl[0].parse::<u32>().unwrap();
    let ydim: u32 = spl[1].parse::<u32>().unwrap();

    return (xdim, ydim);
}




/**
 * Convert raw expanded BHI buffer to a tuple vector of RGB values.
 * @buffer Raw expanded BHI buffer.
 */
pub fn pack_rgb(buffer: &String) -> Vec<(u8, u8, u8)> {

    //Escape dimension metadata.

    let rid_dim: Vec<&str> = buffer.split(":").collect();
    let raw: Vec<&str> = rid_dim[1].split(",").collect();      //This should probably be optimized in some way. 


    //RGB vector.
    let mut rgb: Vec<(u8, u8, u8)> = vec![];

    let mut out_sig = false;


    //Main loop to iterate through BHI buffer.

    for i in raw {

        //Contains 3 values for RGB.
        let value: Vec<&str> = i.split(".").collect();
        let mut temp = vec![]; //Temporary, until read-only tuple push to main vector.

        for j in value {

            //Catch formatting error.

            temp.push( match j.parse::<u8>() {
                Err(_) => {
                    if j.len() == 0 {
                        out_sig = true;
                        break;
                    } panic!("BHI Formatting error. {}", j);
                },
                Ok(j) => j
            } );
        }

        //Between terminating comma EOF.
        if out_sig {
            break;
        }

        rgb.push((temp[0], temp[1], temp[2]));
        
    }

    return rgb;
}



/**
 * Expand buffer and de-vectorize buffer.
 * @file File.
 */
pub fn to_raw_bhi(file:&mut File) -> String {
    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data);


    let mut buf = Vec::new();
    let cpy: &[u8] = &Vec::from(data);

    snap::read::FrameDecoder::new(cpy).read_to_end(&mut buf).unwrap();

    println!("Read and de-compressed {} bytes of data.", &cpy.len());

    let decomp_buffer:String = buf.iter().map(move |b| *b as char).collect();

    return decomp_buffer;
}


/**
 * Raw function to convert from BHI to other supported file formats.
 * @file_path Path to BHI file.
 * @new_name Out-file name.
 * @format New format.
 */
pub fn from_BHI(file_path: &String, new_name: &String, format: &String) {

    //Open file.

    let mut file = match File::open(file_path) {
        Err(err) => {
            println!("Error: BHI file at '{}' not found. {}", file_path, err);
            return;
        }
        Ok(decoded) => {
            println!("File successfully read.");
            decoded
        }
    };

    
    let decomp_buffer: String = to_raw_bhi(&mut file);

    println!("Encoded {} bytes of vectorized data.", &decomp_buffer.len());


    //Dimensional metadata.
    let dimensions:(u32, u32) = get_dimensions(&decomp_buffer);
    let xmax: u32 = dimensions.0;
    let ymax: u32 = dimensions.1;


    //Serialize RGB into vector.
    let rgb: Vec<(u8, u8, u8)> = pack_rgb(&decomp_buffer);

    //New image.
    let mut img = RgbImage::new(xmax, ymax);



    //Iterate through each expected pixel and pull it from vector.

    for i in 0..xmax {
        println!("{}% ({} pixels complete)", ((((i*ymax) as f32) / ((ymax*xmax) as f32)) * 100 as f32).round() as i32, i*ymax);


        for j in 0..ymax {

            //Index of vector.
            let index: u32;       
            
            //Fix logic error with formula.
            if j == 0 {
                index = 0;
            } else {

                //Determines how far from the bottom of the image any given point is, based on the X and Y coordinates on the image.
                index = j*xmax - (xmax-i);
            }

            let cp = rgb[ index as usize ];

            //Write to image.
            img.put_pixel(i, j, Rgb([cp.0, cp.1, cp.2]));
        }
    }

    println!("Mapped {}x{} image.", xmax, ymax);

    //Save to files.
    img.save(&format!("{}.{}", new_name, format));

    println!("Saved file '{}.{}'.", new_name, format);
}   


/**
 * Top-level interfacing function for converting from BHI to other supported image formats.
 * @file_path Path to BHI file.
 * @new_name Out-file name.
 * @format New format.
 */
pub fn convert_from_BHI(file_path: String, new_name: String, format: String) {

    let ext = format.as_str();

    if ext == "jpg" || ext == "png" || ext == "ico" || ext == "bmp" || ext == ".tiff" {
        from_BHI(&file_path, &new_name, &format);
    } else {
        println!("Error: Unsupported file type. File types supported are; JPEG, PNG, ICO, BMP, and TIFF.");
        return;
    }
}