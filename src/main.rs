mod structural;

use structural::factory::{render,window};



fn main() {
    let windows = true;
    if windows{
        render::render(window::WindowFactory)
    } else {
        println!("Nothing to do ")
    }
}
