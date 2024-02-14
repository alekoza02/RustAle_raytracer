use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod utils;
use utils::file::write_ppm;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const ZONE_COUNT: usize = 12;

fn render_zone(start_x: usize, end_x: usize, start_y: usize, end_y: usize, output: Arc<Mutex<Vec<u8>>>, i : usize) {
    // Render the specified zone
    // This function will render pixels from start_x to end_x and start_y to end_y
    // Implement your ray tracing logic here
    // For demonstration purposes, let's just fill the zone with a color (e.g., red)
    
    let mut output = output.lock().unwrap();
    for y in start_y..end_y {
        for x in start_x..end_x {
            let index = (y * WIDTH + x) * 3;
            output[index] = (255 - 255 * i / ZONE_COUNT) as u8; // Red component
            output[index + 1] = (255 * i / ZONE_COUNT) as u8; // Green component
            output[index + 2] = 255; // Blue component
        }
    }
}

fn save_output(output: &Arc<Mutex<Vec<u8>>>, sleep_duration : u64, stop_flag: Arc<Mutex<bool>>) {
    // Simulate saving the output to a file every 5 seconds
    loop {
        // Check the stop flag to see if the thread should exit
        if *stop_flag.lock().unwrap() {
            break;
        }
        // Call a function to save the output to a file
        let output_clone = Arc::clone(&output);
        let locked_data = output_clone.lock().unwrap();
        let cloned_vec: Vec<u8> = locked_data.clone();

        save_output_to_file(&cloned_vec);

        // Sleep for 5 seconds
        thread::sleep(Duration::from_secs(sleep_duration));
    }
}

fn save_output_to_file(output: &[u8]) {
    // Function to save the output vector to a file
    // Implement your file-saving logic here
    let _ = write_ppm("OUTPUT/test_multi.ppm", output, WIDTH as i32, HEIGHT as i32, 255);
}

fn main() {
    // Initialize the output vector
    let output = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT * 3]));

    // Create a flag to signal the output thread to stop
    let stop_flag = Arc::new(Mutex::new(false));

    // Create threads for rendering zones
    let mut handles = vec![];

    let zone_width = WIDTH / 4;
    let zone_height = HEIGHT / 3;

    for i in 0..ZONE_COUNT {
        let start_x = (i % 4) * zone_width;
        let end_x = start_x + zone_width;
        let start_y = (i / 4) * zone_height;
        let end_y = start_y + zone_height;

        let output_clone = Arc::clone(&output);
        let handle = thread::spawn(move || {
            render_zone(start_x, end_x, start_y, end_y, output_clone, i);
        });

        handles.push(handle);
        println!("Lanciato il {}!", i)
    }


    // Create a thread for saving output to file
    let output_clone = Arc::clone(&output);
    let stop_flag_clone = Arc::clone(&stop_flag);
    let output_thread = thread::spawn(move || {
        save_output(&output_clone, 1, stop_flag_clone);
    });


    // Wait for all rendering threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Set the stop flag to true to signal the output thread to stop
    *stop_flag.lock().unwrap() = true;

    // Wait for the output thread to finish
    output_thread.join().unwrap();

    let output_clone = Arc::clone(&output);
    let locked_data = output_clone.lock().unwrap();
    let cloned_vec: Vec<u8> = locked_data.clone();

    save_output_to_file(&cloned_vec);

}
