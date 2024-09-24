# rust-go
The classic game of Go written in Rust

## Development

### Development Requirements

The following dependencies are required for development with bevy.

sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0

### Building

cargo build

## Running

### Dynamic linking
This is the most impactful compilation time decrease! 
You can compile bevy as dynamic library, preventing it from having to be statically linked each time you rebuild 
your project. You can enable this with the dynamic_linking feature flag.
https://bevyengine.org/learn/quick-start/getting-started/setup/

cargo run --features "bevy/dynamic_linking"
