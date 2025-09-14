# rust-go
The classic game of Go written in Rust using the Bevy game engine.

## Development

### Development Requirements

The following dependencies are required for development with bevy:

```bash
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
```

### Building

```bash
cargo build
```

### Testing

Run the test suite with:

```bash
cargo test
```

## Running

### Dynamic linking
You can compile bevy as dynamic library, preventing it from having to be statically linked each time you rebuild 
your project. You can enable this with the dynamic_linking feature flag.
https://bevyengine.org/learn/quick-start/getting-started/setup/

```bash
cargo run --features "bevy/dynamic_linking"
```

## Project Structure
- `src/main.rs` - Application entry point and Bevy setup
- `src/game.rs` - Core game logic and rules implementation
- `src/grid.rs` - Board visualization and interaction handling
