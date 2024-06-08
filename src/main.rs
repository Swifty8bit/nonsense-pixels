use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer};

const PATH: &str = "images/image.png";
const CHUNK_SIZE: u32 = 16;

fn main() 
{
    let mut img: DynamicImage = ImageReader::open(PATH).unwrap().decode().unwrap();

    let gradient: DynamicImage = ImageReader::open("assets/gradient.png").unwrap().decode().unwrap();

    println!("{}", img.height());
    println!("{}", img.width());

    let image_size_x = (img.height() / CHUNK_SIZE) * CHUNK_SIZE;
    let image_size_y = (img.width() / CHUNK_SIZE) * CHUNK_SIZE;

    let mut imgbuf: ImageBuffer<image::Rgb<u8>, Vec<u8>> = 
        image::ImageBuffer::new(image_size_y, image_size_x);

    // grayscale
    img = img.grayscale();

    // analyze the image
    let mut image_data: Vec<Vec<ImageData>> = Vec::new();
    // [[ImageData; image_size_y]; image_size_x];

    for n in 0..(image_size_x / CHUNK_SIZE)
    {
        let mut inner_vec: Vec<ImageData> = Vec::new();
        
        for m in 0..(image_size_y / CHUNK_SIZE)
        {
            inner_vec.push(scan_chunk(n as usize, m as usize, &img));
        }
        image_data.push(inner_vec);
    }
 
    for n in image_data.iter_mut()
    {
        for m in n.iter_mut()
        {
            // check ifs and draw square
            draw_square(m, &mut imgbuf, &gradient);
        }
    }

    // save file
    imgbuf.save("images/result.png").unwrap();
}

fn scan_chunk(_x: usize, _y: usize, _img: &DynamicImage) -> ImageData
{
    let mut all_values: u32 = 0;

    for n in 0..CHUNK_SIZE
    {
        for m in 0..CHUNK_SIZE
        {
            all_values += _img.get_pixel((_y * CHUNK_SIZE as usize + m as usize) as u32, (_x * CHUNK_SIZE as usize + n as usize) as u32).0[0] as u32;
        }
    }

    all_values = all_values / (CHUNK_SIZE * 16);
    let brightness_calculated = all_values as u8;

    let imgdt = ImageData
    {
        x: _x,
        y: _y,
        brightness: brightness_calculated
    };

    return imgdt;
}

fn draw_square(_imgdt: &ImageData, _imgbuf: 
    &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, 
    _gradient: &DynamicImage)
{

    for n in 0..CHUNK_SIZE
    {
        for m in 0..CHUNK_SIZE
        {
            let pixel = _imgbuf.get_pixel_mut(
                (_imgdt.y * CHUNK_SIZE as usize + m as usize) as u32, 
                (_imgdt.x * CHUNK_SIZE as usize + n as usize) as u32);
            
            let color = _gradient.get_pixel(
                index_from_brightness(_imgdt.brightness) + n as u32, 
                m as u32).0[0];
            
            pixel.0[0] = color;
            pixel.0[1] = color;
            pixel.0[2] = color;
        }
    }
}

fn index_from_brightness(_value: u8) -> u32
{
    if _value < 16
    {
        return 0;
    }
    else if _value < 32
    {
        return 16;
    }
    else if _value < 48 
    {
        return 32;
    }
    else if _value < 64
    {
        return 48;
    }
    else if _value < 80
    {
        return 64;
    }
    else if _value < 96
    {
        return 80;
    }
    else if _value < 112
    {
        return 96;
    }
    else if _value < 128
    {
        return 112;
    }
    else if _value < 144
    {
        return 128;
    }
    else if _value < 160
    {
        return 144;
    }
    else if _value < 176
    {
        return 160;
    }
    else if _value < 192
    {
        return 176;
    }
    else if _value < 208
    {
        return 192;
    }
    else if _value < 224
    {
        return 208;
    }
    else if _value < 240
    {
        return 224;
    } 
    return 240;
}

struct ImageData
{
    x: usize,
    y: usize,
    brightness: u8
}