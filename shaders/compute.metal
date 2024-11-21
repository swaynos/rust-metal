#include <metal_stdlib>
using namespace metal;

kernel void crop_and_resize(const device float4 *input_image [[buffer(0)]],
                            device float *output_image [[buffer(1)]],
                            constant uint &width [[buffer(2)]],
                            constant uint &height [[buffer(3)]],
                            constant uint &new_width [[buffer(4)]],
                            constant uint &new_height [[buffer(5)]],
                            constant uint &crop_top [[buffer(6)]],
                            constant uint &crop_bottom [[buffer(7)]],
                            uint id [[thread_position_in_grid]]) {
    // Compute new pixel coordinates
    uint original_x = id % new_width;
    uint original_y = (id / new_width) % new_height;

    // Map original coordinates from the resized image
    float x_scale = float(width) / float(new_width);
    float y_scale = float(height - (crop_top + crop_bottom)) / float(new_height);

    // Calculate the corresponding pixel in the input image
    uint input_x = uint(original_x * x_scale);
    uint input_y = uint(crop_top + original_y * y_scale);

    // Read from input image and write to output image
    float pixel = input_image[input_y * width + input_x].r; // Use red channel for single-channel output
    output_image[id] = pixel;
}