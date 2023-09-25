use rand::Rng;
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;


pub struct AllStars {
    the_stars: Vec<AStar>,
    window_width: i32,
    window_height: i32,
}

impl AllStars {
    pub fn new(window_width: i32, window_height: i32) ->  AllStars {
        AllStars {
            window_height: window_height,
            window_width: window_width,
            the_stars: Vec::new(),
        }
    }

    /// Populates a vector with a collection of AStar objects
    pub fn populate_star(&mut self, window_width: i32, window_height: i32) -> () {
        let mut star_index: usize = 0;
        while star_index < NUMBER_OF_STARS {
            self.the_stars.push(AStar::new(window_width, window_height));
            star_index += 1;
        }
    }

    /// Moves the stars in the vector, as per their current speeds.
    pub fn move_stars(&mut self) -> () {
        let mut rng = rand::thread_rng();
        let mut star_index: usize = 0;
        while star_index < NUMBER_OF_STARS {
            let mut new_x: i32 =
                self.the_stars[star_index].x as i32 + self.the_stars[star_index].speed;
            if new_x < 0 {
                new_x = self.window_width;
                self.the_stars[star_index].y = rng.gen_range(0..self.window_height);
                self.the_stars[star_index].speed =
                    -((rng.gen_range(MIN_SPEED..MAX_SPEED) & 0xFFF) as i32);
            }
            self.the_stars[star_index].x = new_x;
            star_index += 1;
        }
    }

    /// Plot stars in the vector.
    pub fn plot_stars(self) -> () {
        for a_star in self.the_stars {
            let (the_colour, layer) = a_star.get_colour_and_layer_from_speed();
            println!("{:?} {:?}", the_colour, layer);
        }
    }

    pub fn set_window_size(&mut self, window_width: i32, window_height: i32)
    {
        self.window_height = window_height;
        self.window_width = window_width;
    }
}






fn main() {
    let mut all_stars = AllStars::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    all_stars.set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut silly_number: i64 = 0;
    
    while silly_number < 34324323  {
        all_stars.move_stars();
        all_stars.set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);
        all_stars.plot_stars();
        silly_number += 1;
    }
}




/* Star Junk */
use std::fmt;

static MIN_SPEED: u32 = 4;
static MAX_SPEED: u32 = 14;
const NUMBER_OF_STARS: usize = 250;

/// Strcuture to store star information.
pub struct AStar {
    x: i32,
    y: i32,
    speed: i32,
}
impl fmt::Debug for AStar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AStar")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("speed", &self.speed)
            .finish()
    }
}

impl AStar {
    /// Create a new Star. Will generate a star with a location
    /// inside window_width and window_height and a speed
    /// as determined by the range MIN_SPEED/MAX_SPEED constants
    pub fn new(window_width: i32, window_height: i32) -> AStar {
        let mut rng = rand::thread_rng();
        let local_speed = rng.gen_range(MIN_SPEED..MAX_SPEED) & 0xFFF;
        let signed_speed = -(local_speed as i32);
        AStar {
            x: rng.gen_range(0..window_width),
            y: rng.gen_range(0..window_height),
            speed: signed_speed,
        }
    }

    /// Takes the current speed of the star and works out the colour it should be
    /// and also the "speed" layer that it is in.
    fn get_colour_and_layer_from_speed(&self) -> (i32, i32) {
        let speed_range = (MAX_SPEED - MIN_SPEED) as f32;
        let layers = 4;
        let speed_to_test = self.speed.abs() as f32;
        let speed_increment_in_layer: f32 = speed_range as f32 / layers as f32;

        let mut current_bottom_layer_value = 0.0;

        let mut layer = 0;
        while speed_to_test > current_bottom_layer_value {
            current_bottom_layer_value += speed_increment_in_layer;
            layer += 1;
        }

        // Here we know the layer that our current speed falls in.
        let level_value = 255.0 / ((layers - layer) + 1) as f32;

        // Return a tuple to our caller with the Colour and the Layer
        (
            level_value as i32,
            layer,
        )
    }
}

