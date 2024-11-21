# rust-metal
Playing around with Rust and [Apple's Metal API](https://developer.apple.com/documentation/metal/performing_calculations_on_a_gpu?language=objc).

### Compiling Metal Shaders
To recompile the Metal shaders, ensure you have Xcode Command Line Tools installed, then run:
```./compile_shaders.sh```
- As a workaround, with Command Line Tools installed, you can install XCode (full macOS IDE) and then run this before compiling the shaders: ```export PATH="/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin:$PATH"```
- Then clear this from path and run ```cargo build``` to build the Rust application