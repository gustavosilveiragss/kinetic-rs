# Kinetic Mouse Stabilizer

Kinetic is an accessibility-focused mouse movement stabilizer that helps people with hand tremors, motor control challenges, and other movement-related accessibility needs use their computer more effectively. The application works system-wide on both Windows and Linux, creating a smoother and more controllable cursor experience by processing mouse input. Future development may include additional I/O accessibility features - suggestions are welcome.

## How It Works

Kinetic runs in the background, analyzing mouse movement in real-time through several steps:

- Capturing raw mouse input
- Identifying intended movement direction
- Filtering out unintended movement and tremors
- Maintaining responsiveness while smoothing motion
- Adapting to different movement speeds and patterns

While originally developed for accessibility purposes, accessibility features could benefit anyone seeking cursor precision. However, our primary focus remains on accessibility and creating an inclusive computing experience.

## Features in Development

Our core focus is on accessibility, with planned features including:

- Cross-platform support (Windows and Linux)
- Real-time movement stabilization
- Customizable stabilization profiles
- Minimal resource usage
- Low latency processing
- System-wide application
- Additional input/output accessibility options (planned)

## Building from Source

Currently, no release builds have been made, so the project must be built from source. Here's what you'll need:

### Prerequisites

- Rust toolchain
- Git
- Linux users: X11 development libraries

### Build Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/gustavosilveiragss/kinetic-rs
   cd kinetic-rs
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run with debug logging (helpful for development):

   ```bash
   # On Linux:
   export RUST_LOG=debug
   ./target/release/kinetic-rs

   # On Windows (CMD):
   set RUST_LOG=debug
   .\target\release\kinetic-rs.exe

   # On Windows (PowerShell):
   $env:RUST_LOG = "debug"
   .\target\release\kinetic-rs.exe
   ```

## Contributing

We welcome contributions, particularly those that help make computing more accessible. Whether it's:

- Accessibility feature suggestions
- Code improvements - Code improvements (Note: Rust amateur, so all help is welcome)
- Documentation updates

Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---
