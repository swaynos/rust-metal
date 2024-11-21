use metal::*;
use std::mem;

#[repr(C)]
struct Params {
    width: u32,
    height: u32,
    new_width: u32,
    new_height: u32,
    crop_top: u32,
    crop_bottom: u32,
}

fn main() {
    // Initialize Metal
    let device = Device::system_default().expect("Failed to find a Metal device");
    println!("Using device: {}", device.name());

    // Load the Metal shader
    let library = device.new_library_with_file("shaders/compute.metallib").unwrap();
    let function = library.get_function("crop_and_resize", None).unwrap();
    let pipeline_state = device
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    // Dimensions of the original image
    let width = 5120;
    let height = 2880;
    let new_width = 2560;
    let new_height = 1440;
    let crop_top = 105;
    let crop_bottom = 105;

    // Create the parameters buffer
    let params = Params {
        width,
        height,
        new_width,
        new_height,
        crop_top,
        crop_bottom,
    };

    let params_buffer = device.new_buffer_with_data(
        &params as *const _ as *const std::ffi::c_void,
        std::mem::size_of::<Params>() as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );

    // Placeholder for the input image (for now just zeros)
    let input_image: Vec<[f32; 4]> = vec![[0.0, 0.0, 0.0, 0.0]; (width * height) as usize];
    let mut output_image: Vec<[f32; 4]> = vec![[0.0, 0.0, 0.0, 0.0]; (new_width * new_height) as usize];

    // Create input buffer for the image
    let buffer_input = device.new_buffer_with_data(
        input_image.as_ptr() as *const _,
        (input_image.len() * mem::size_of::<[f32; 4]>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );

    // Create output buffer
    let buffer_output = device.new_buffer(
        (output_image.len() * mem::size_of::<[f32; 4]>()) as u64,
        MTLResourceOptions::StorageModeShared,
    );

    // Create a command queue and encoder
    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_compute_command_encoder();

    // Set the shader parameters
    encoder.set_compute_pipeline_state(&pipeline_state);
    encoder.set_buffer(0, Some(&buffer_input), 0); // Input image buffer
    encoder.set_buffer(1, Some(&buffer_output), 0); // Output image buffer
    encoder.set_buffer(2, Some(&params_buffer), 0); // Scalar parameters buffer

    // Dispatch threads
    let threadgroup_size = MTLSize::new(256, 1, 1);
    let threadgroup_count = MTLSize::new(
        ((new_width * new_height) as u64 + threadgroup_size.width - 1) / threadgroup_size.width,
        1,
        1,
    );

    encoder.dispatch_thread_groups(threadgroup_count, threadgroup_size);
    encoder.end_encoding();

    // Commit and wait for completion
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Read back the output image
    let output_ptr = buffer_output.contents() as *const [f32; 4];
    for i in 0..output_image.len() {
        output_image[i] = unsafe { *output_ptr.add(i) };
    }

    // Display or process output_image as needed
    println!("First 10 resized pixel values: {:?}", &output_image[..10]);
}