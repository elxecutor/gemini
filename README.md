
# Gemini Chat TUI

An awesome Terminal User Interface (TUI) for chatting with Google's Gemini AI! Built with Rust and featuring a colorful, animated interface.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Setup](#setup)
- [Usage](#usage)
- [Demo Mode](#demo-mode)
- [UI Elements](#ui-elements)
- [Technical Details](#technical-details)
- [Configuration](#configuration)
- [API Usage](#api-usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- **Rainbow animated title** with cycling colors
- **Beautiful chat bubbles** for user and AI messages
- **Real-time streaming** chat experience
- **Colorful UI** with emoji indicators
- **Responsive design** that adapts to your terminal size
- **Loading animations** while waiting for AI responses
- **Persistent configuration** - saves your API key securely
- **Keyboard shortcuts** for smooth navigation

## Installation

1. Make sure you have Rust installed: https://rustup.rs/
2. Clone this repository:
   ```bash
   git clone https://github.com/elxecutor/gemini.git
   cd gemini
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Setup

1. Get your Gemini API key:
   - Go to https://aistudio.google.com/app/apikey
   - Create a new API key
   
2. Run the application:
   ```bash
   cargo run
   ```
   
3. On first run, you'll be prompted to enter your API key. It will be saved for future use.

## Usage

### Keyboard Controls
- **Type** your message and press **Enter** to send
- **Ctrl+C** to quit the application
- **Esc** to cancel a pending message (if Gemini is thinking)
- **Left/Right arrows** to move cursor in input field
- **Backspace** to delete characters

### Command Line Options
```bash
# Set API key from command line
cargo run -- --api-key YOUR_API_KEY

# Reset configuration (will prompt for API key again)
cargo run -- --reset-config

# Run in demo mode (shows beautiful UI without needing API key)
cargo run -- --demo
```

## Demo Mode

Want to see the beautiful TUI without setting up an API key? Run the demo mode:

```bash
cargo run -- --demo
```

This will show you sample conversations with the fixed, perfectly aligned message bubbles!

## UI Elements

- **Rainbow Title**: Animated title bar with cycling colors
- **Chat Area**: 
  - User messages appear in blue bubbles on the right
  - Gemini responses appear in green bubbles on the left
  - Loading animation with spinner while waiting
- **Input Area**: Purple-bordered input field for typing messages
- **Status Bar**: Shows current status and helpful messages

## Technical Details

Built with modern Rust libraries:
- **ratatui**: For the beautiful TUI interface
- **tokio**: For async/await support
- **reqwest**: For HTTP requests to Gemini API
- **crossterm**: For cross-platform terminal handling
- **serde**: For JSON serialization/deserialization

## Configuration

Configuration is stored in:
- Linux: `~/.config/gemini-chat-tui/config.json`
- macOS: `~/Library/Application Support/gemini-chat-tui/config.json`
- Windows: `%APPDATA%\gemini-chat-tui\config.json`

## API Usage

This application uses the Gemini API endpoint:
```
https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent
```

Make sure you have sufficient API quota and follow Google's usage policies.

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest new features
- Submit pull requests
- Improve documentation

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) for details.

## License

This project is open source. Feel free to use, modify, and distribute as needed.

This project is licensed under the [MIT License](LICENSE).

## Contact

For questions or support, please open an issue or contact the maintainer via [X](https://x.com/elxecutor/).