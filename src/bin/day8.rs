use std::fmt;

static TRANSPARENT: i8 = 2;

pub struct Image {
    width: usize,
    height: usize,
    image_layers: Vec<ImageLayer>,
}

impl Image {
    fn parse(pixel_data: &str, width: usize, height: usize) -> Image {
        assert_eq!(pixel_data.len() % (width * height), 0);

        let all_pixels = pixel_data
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap() as i8)
            .collect::<Vec<i8>>();

        let parsed_pixels: Vec<Vec<i8>> = all_pixels
            .chunks(width * height)
            .map(|x| x.to_vec())
            .collect();

        let image_layers = parsed_pixels.iter().fold(Vec::new(), |mut layers, pixels| {
            layers.push(ImageLayer::parse(pixels, width, height));
            layers
        });
        Image { width, height, image_layers }
    }

    pub fn get_image_layer_with_fewest_given_pixel_value(&self, pixel_value: i8) -> &ImageLayer {
        let (_, layer) = self.image_layers
            .iter()
            .fold((0, None), |(previous_layer_count, previous_layer), this_layer| {
                let this_layer_count = this_layer.get_number_of_pixels_matching(pixel_value);
                let no_previous_layer = previous_layer.is_none();
                let this_is_the_new_min = this_layer_count < previous_layer_count;

                if no_previous_layer { return (this_layer_count, Some(this_layer)); }
                if this_is_the_new_min { (this_layer_count, Some(this_layer)) } else { (previous_layer_count, previous_layer) }
            });
        layer.unwrap()
    }

    pub fn get_pixel_color(&self, column_index: usize, row_index: usize) -> i8 {
        let colored_pixel = self.image_layers
            .iter()
            .find(|x| x.pixels[column_index][row_index] != TRANSPARENT);

        if colored_pixel.is_none() { return TRANSPARENT; }
        colored_pixel.unwrap().pixels[column_index][row_index]
    }

    pub fn print(&self) -> String {
        let mut message = vec![vec![TRANSPARENT; self.width]; self.height];
        for column_index in 0..self.height {
            for row_index in 0..self.width {
                message[column_index][row_index] = self.get_pixel_color(column_index, row_index);
            }
        }

        Image::print_pixel(&mut message)
    }

    fn print_pixel(message: &mut Vec<Vec<i8>>) -> String {
        message.iter().fold(String::from(""), |concat, row| {
            let row_string = row.iter().fold(String::from(""), |row_concat, cell| {
                format!("{}{}", row_concat, cell.to_string())
            });
            format!("{}\n{}", concat, row_string)
        })
    }
}

pub struct ImageLayer {
    width: usize,
    height: usize,
    pixels: Vec<Vec<i8>>,
}

impl ImageLayer {
    fn parse(pixels: &Vec<i8>, width: usize, height: usize) -> ImageLayer {
        assert_eq!(width * height, pixels.len());

        let split_pixels = pixels
            .chunks(width)
            .map(|x| x.to_vec())
            .collect();

        ImageLayer { width, height, pixels: split_pixels }
    }

    fn get_number_of_pixels_matching(&self, num: i8) -> usize {
        self.pixels.iter().fold(0, |count, row| {
            count + row.into_iter().fold(0, |row_count, cell| {
                if *cell == num { row_count + 1 } else { row_count }
            })
        })
    }

    fn get_number_of_pixels_not_matching(&self, num: i8) -> usize {
        self.pixels.iter().fold(0, |count, row| {
            count + row.into_iter().fold(0, |row_count, cell| {
                if *cell != num { row_count + 1 } else { row_count }
            })
        })
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.image_layers.iter().for_each(|x| {
            write!(f, "{}", x.to_string());
        });
        Ok(())
    }
}

impl fmt::Display for ImageLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.pixels)
    }
}


pub fn solution(image: &Image) -> i32 {
    let layer_with_min_0_pixel = image.get_image_layer_with_fewest_given_pixel_value(0);
    (layer_with_min_0_pixel.get_number_of_pixels_matching(1)
        * layer_with_min_0_pixel.get_number_of_pixels_matching(2)) as i32
}

pub fn main() {
    let contents = include_str!("../../data/eight.data");

    println!("Print: {}", Image::parse(contents, 25, 6).print())
}


#[cfg(test)]
mod tests {
    use crate::{Image, solution};

    #[test]
    fn parses_pixels() {
        assert_eq!(Image::parse("123456111222", 3, 2).to_string(), "[[1, 2, 3], [4, 5, 6]][[1, 1, 1], [2, 2, 2]]");
    }

    #[test]
    fn gets_count_of_matching_pixels() {
        assert_eq!(Image::parse("100006", 3, 2).image_layers[0].get_number_of_pixels_matching(0), 4);
    }

    #[test]
    fn gets_count_of_not_matching_pixels() {
        assert_eq!(Image::parse("100006", 3, 2).image_layers[0].get_number_of_pixels_not_matching(0), 2);
    }

    #[test]
    fn get_layer_with_fewest_0() {
        assert_eq!(
            Image::parse("103456111222", 3, 2).get_image_layer_with_fewest_given_pixel_value(0).to_string(),
            "[[1, 1, 1], [2, 2, 2]]");
    }

    #[test]
    fn it_works() {
        assert_eq!(
            solution(&Image::parse("100456101222", 3, 2)), 6);
    }

    #[test]
    fn gets_message() {
        assert_eq!(
            Image::parse("0222112222120000", 2, 2).print(), "0110");
    }
}