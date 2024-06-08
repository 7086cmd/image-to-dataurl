use std::fs::File;
use std::io::Read;
use base64::encode;

enum ImageType {
    PNG,
    SVG,
    JPEG,
    GIF,
    WEBP,
    BMP,
    ICO,
    TIFF,
    UNKNOWN,
}

impl ImageType {
    fn from_path(path: &str) -> Self {
        match path {
            _ if path.ends_with(".png") => ImageType::PNG,
            _ if path.ends_with(".svg") => ImageType::SVG,
            _ if path.ends_with(".jpg") => ImageType::JPEG,
            _ if path.ends_with(".jpeg") => ImageType::JPEG,
            _ if path.ends_with(".gif") => ImageType::GIF,
            _ if path.ends_with(".webp") => ImageType::WEBP,
            _ if path.ends_with(".bmp") => ImageType::BMP,
            _ if path.ends_with(".ico") => ImageType::ICO,
            _ if path.ends_with(".tiff") => ImageType::TIFF,
            _ => ImageType::UNKNOWN,
        }
    }
    fn convert_to_mime(&self) -> &'static str {
        match self {
            ImageType::PNG => "image/png",
            ImageType::SVG => "image/svg+xml",
            ImageType::JPEG => "image/jpeg",
            ImageType::GIF => "image/gif",
            ImageType::WEBP => "image/webp",
            ImageType::BMP => "image/bmp",
            ImageType::ICO => "image/x-icon",
            ImageType::TIFF => "image/tiff",
            ImageType::UNKNOWN => "application/octet-stream",
        }
    }
    fn encode_bits(&self, data: &str) -> String {
        let mut file = File::open(data).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        match self {
            ImageType::SVG => encode_svgs(data).unwrap(),
            _ => format!("export default 'data:{};base64,{}';", self.convert_to_mime(), encode(&buffer)),
        }
    }
    fn encode_svg(data: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Read the SVG file
        let mut file = File::open(data)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        // Construct the data URL
        let data_url = format!("data:image/svg+xml,{}", urlencoding::encode(&buffer));

        // Then convert to JavaScript module
        Ok(["export default '", data_url.as_str(), "';"].concat())
    }
}

fn main() {
    // println!("Hello, world!");
    let src = "assets/hsr.svg";
    let image_type = ImageType::from_path(src);
    let data = image_type.encode_bits(src);
    println!("{}", data);
}

fn encode_svgs(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Read the SVG file
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Construct the data URL
    let data_url = format!("data:image/svg+xml,{}", urlencoding::encode(&buffer));

    // Then convert to JavaScript module
    Ok(["export default '", data_url.as_str(), "';"].concat())
}