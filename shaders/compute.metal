#include <metal_stdlib>
using namespace metal;

kernel void add_arrays(const device float *array1 [[buffer(0)]],
                       const device float *array2 [[buffer(1)]],
                       device float *result [[buffer(2)]],
                       uint id [[thread_position_in_grid]]) {
    result[id] = array1[id] + array2[id];
}
