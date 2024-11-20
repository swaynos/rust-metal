use metal::*;
use std::mem;

fn main() {
    // Initialize Metal
    let device = Device::system_default().expect("Failed to find a Metal device");
    println!("Using device: {}", device.name());

    // Load the Metal shader
    let library = device.new_library_with_file("shaders/compute.metallib").unwrap();
    let function = library.get_function("add_arrays", None).unwrap();
    let pipeline_state = device
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    // Create buffers
    let array_size = 1024;
    let array1: Vec<f32> = (0..array_size).map(|x| x as f32).collect();
    let array2: Vec<f32> = (0..array_size).map(|x| x as f32).collect();
    let mut result = vec![0.0f32; array_size];

    let buffer1 = device.new_buffer_with_data(
        array1.as_ptr() as *const _,
        (array_size * mem::size_of::<f32>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );
    let buffer2 = device.new_buffer_with_data(
        array2.as_ptr() as *const _,
        (array_size * mem::size_of::<f32>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );
    let result_buffer = device.new_buffer(
        (array_size * mem::size_of::<f32>()) as u64,
        MTLResourceOptions::StorageModeShared,
    );

    // Create a command queue and encoder
    let command_queue = device.new_command_queue();
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_compute_command_encoder();

    encoder.set_compute_pipeline_state(&pipeline_state);
    encoder.set_buffer(0, Some(&buffer1), 0);
    encoder.set_buffer(1, Some(&buffer2), 0);
    encoder.set_buffer(2, Some(&result_buffer), 0);

    // Dispatch threads
    let threadgroup_size = MTLSize::new(16, 1, 1);
    let threadgroup_count = MTLSize::new(
        (array_size as u64 + threadgroup_size.width - 1) / threadgroup_size.width,
        1,
        1,
    );
    
    

    encoder.dispatch_thread_groups(threadgroup_count, threadgroup_size);
    encoder.end_encoding();

    // Commit and wait for completion
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Read the result
    let result_ptr = result_buffer.contents() as *const f32;
    for i in 0..array_size {
        result[i] = unsafe { *result_ptr.add(i) };
    }

    println!("First 10 results: {:?}", &result[..10]);
}
