xcrun -sdk macosx metal -c shaders/compute.metal -o shaders/compute.air
xcrun -sdk macosx metallib shaders/compute.air -o shaders/compute.metallib