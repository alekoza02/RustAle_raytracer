const IMAGE_SIZE: usize = 1080;
const CHANNELS: usize = 3;

fn main() {
    // Create a mutable array to store the image data
    let mut image_data: [u8; IMAGE_SIZE * IMAGE_SIZE * CHANNELS] = [0; IMAGE_SIZE * IMAGE_SIZE * CHANNELS];

    // Access and modify elements of the array as needed
    for i in 0..IMAGE_SIZE {
        for j in 0..IMAGE_SIZE {
            // Calculate the index for the current pixel
            let index = (i * IMAGE_SIZE + j) * CHANNELS;

            // Example: Assigning RGB values to the pixel
            image_data[index] = 255;    // Red channel
            image_data[index + 1] = 0;  // Green channel
            image_data[index + 2] = 0;  // Blue channel
        }
    }

    println!("Dinito")
}
