use std::mem::size_of;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct PixelBuffer {
    pub pixels: Vec<u8>,
    pitch: usize,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    pub fn new<T>(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0xFF_u8; width * height * size_of::<T>()],
            pitch: width * size_of::<u32>(),
            width,
            height,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    // pub fn load_texture(name: String) -> Self {
    //     let image = image::open(format!("./images/{}", name)).unwrap();
    //     PixelBuffer::from_texture::<u32>(
    //         image.as_bytes(),
    //         image.width() as usize,
    //         image.height() as usize,
    //     )
    // }

    pub fn from_texture<T>(texture: &[u8], width: usize, height: usize) -> Self {
        Self {
            pixels: texture.to_vec(),
            pitch: width * size_of::<T>(),
            width,
            height,
        }
    }

    pub fn get_pitch(&self) -> usize {
        self.pitch
    }

    pub fn get_pixel_at(&self, x: i32, y: i32) -> u32 {
        let index = (y * self.pitch as i32 + x * 4) as usize;

        if index < self.pixels.len() {
            let data = [
                self.pixels[index + 3],
                self.pixels[index + 2],
                self.pixels[index + 1],
                self.pixels[index],
            ];
            ((data[0] as u32) << 24)
                + ((data[1] as u32) << 16)
                + ((data[2] as u32) << 8)
                + ((data[3] as u32) << 0)
        } else {
            println!("Pixel not found at {x}, {y}");
            0x5ea4ccFF_u32
        }
    }

    pub fn set_pixel_at(&mut self, x: i32, y: i32, color: u32) {
        let i = (y * self.pitch as i32 + x * 4) as usize;
        if i < self.pixels.len() {
            self.set(i, color);
        }
    }

    pub fn update_buffer<F>(&mut self, func: F)
    where
        F: Fn(usize, usize) -> u32,
    {
        for x in 0..self.width {
            for y in 0..self.height {
                let i = y * self.pitch + x * 4;
                self.set(i, func(x, y));
            }
        }
    }

    pub fn clear_buffer(&mut self, color: u32) {
        for x in 0..self.width * self.height {
            let i = x * 4;
            self.set(i, color);
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for rect_x in x..=(width + x) {
            for rect_y in y..=(height + y) {
                self.set_pixel_at(rect_x, rect_y, color);
            }
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let delta_x = x2 - x1;
        let delta_y = y2 - y1;

        let side_length = if delta_x.abs() > delta_y.abs() {
            delta_x.abs()
        } else {
            delta_y.abs()
        };

        let x_inc = delta_x as f32 / side_length as f32;
        let y_inc = delta_y as f32 / side_length as f32;

        let mut current_x = x1 as f32;
        let mut current_y = y1 as f32;

        for _ in 0..=side_length {
            self.set_pixel_at(current_x.round() as i32, current_y.round() as i32, color);
            current_x += x_inc;
            current_y += y_inc;
        }
    }

    fn set(&mut self, index: usize, color: u32) {
        let bytes = color.to_be_bytes();
        self.pixels[index] = bytes[0]; // BLUE
        self.pixels[index + 1] = bytes[1]; // GREEN
        self.pixels[index + 2] = bytes[2]; //RED

        self.pixels[index + 3] = bytes[3]; // ALPHA
    }
}
