# PhantomKeystroke 

## Overview
**PhantomKeystroke v1.0.0** is an open-source, pure Rust application that turns attribution into a game of deception. By weaving subtle, region-specific fingerprints into your commands, scripts, and keystrokes, it misleads forensic analysts while keeping your operations intact. It adds barely perceptible regional markers to command-line operations that can mislead forensic attribution efforts while maintaining full command functionality. PhantomKeystroke can operate as a standalone terminal or as a proxy for Command and Control (C2) frameworks, offering two primary modes: **Random Mode** for unpredictable obfuscation and **Attribute Mode** for region-specific emulation—perfect for red teaming, research, or understanding the art of cyber misattribution.

![3971C9AB-5682-4E29-B2EA-B41E86186D0F](https://github.com/user-attachments/assets/ab22cb42-4b08-43b1-b160-c532dba5707f)


## Core Features

- **Subtle Attribution Fingerprinting**: Adds barely perceptible regional markers to your commands that would lead an analyst to attribute activity to a specific country
- **Realistic Keyboard Patterns**: Simulates typing patterns of operators from different regions with realistic timing and jitter
- **Command Functionality Preservation**: All commands remain 100% functional - only subtle fingerprints are added
- **Multi-Region Support**: Attribution to multiple countries including Russia, China, Korea, Iran, Arabic countries, Germany, and France
- **Plugin System**: Integrates with C2 frameworks through a flexible plugin interface

## Quick Start

```bash
# Install dependencies
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Windows (PowerShell)
# Invoke-WebRequest https://win.rustup.rs -OutFile rustup-init.exe
# .\rustup-init.exe
# Remove-Item rustup-init.exe

# Clone the repository
git clone https://github.com/JAYGLXR/phantomkeystroke.git
cd phantomkeystroke

# Build the application
cargo build --release

# Run with default settings (Russian attribution)
./target/release/phantom_keystroke --random --plugin=null

# Run with Chinese attribution
./target/release/phantom_keystroke --random --plugin=null --attribution zh

# Run with verbose output (shows each keystroke)
./target/release/phantom_keystroke --random --plugin=null --verbose
```

## Attribution Fingerprinting

PhantomKeystroke adds subtle attribution markers based on the specified target region:

- **Russian**: Occasional transliterated variable names (e.g., `privet_world`), Cyrillic 'е' character usage in comments, keyboard layout artifacts
- **Chinese**: Use of pinyin variable names (e.g., `nihao_function`), full-width spaces (　), date format adjustments (YYYY年MM月DD日)
- **Korean**: Hangul character markers (e.g., `안녕_variable`), specific variable naming conventions
- **Persian/Farsi**: RTL markers, Persian number formats (۱۲۳), transliterated names (e.g., `salam_world`)
- **Arabic**: Arabic comma substitution (،), Arabic numbers (١٢٣), variable naming patterns
- **German**: Y/Z character swaps (keyboard layout artifact - e.g., `zellow` instead of `yellow`), date formatting (DD.MM.YYYY)
- **French**: AZERTY keyboard slips (e.g., `qzerty_function`), spacing before punctuation marks (space before !)

These subtle elements create a pattern that can mislead attribution efforts while maintaining the full functionality of your commands.

## Operational Modes

### Random Mode (Default)
- Applies subtle attribution fingerprints from the selected region
- Simulates realistic typing patterns with appropriate pauses and jitter
- Displays simplified output by default (use `--verbose` to see all details)

### Attribute Mode
- Creates a comprehensive regional persona including keyboard patterns, language traits, and timestamp characteristics
- OPSEC validation to ensure operational times match the target region
- Full emulation of regional attributes for extended operations

## Command Line Options

| Option | Description | Example Values |
|--------|-------------|----------------|
| `-c, --config <FILE>` | Use a configuration file | `config.toml` |
| `-r, --random` | Run in Random Mode | N/A |
| `-a, --attribute` | Run in Attribute Mode | N/A |
| `-p, --plugin <NAME>` | Use a specific plugin | `null`, `cobaltstrike`, `sliver` |
| `-l, --log-file` | Log to file instead of stdout | N/A |
| `-v, --verbose` | Run in verbose mode (shows keystrokes) | N/A |
| `-t, --attribution <COUNTRY>` | Target country for fingerprinting | `ru`, `zh`, `random` |
| `-h, --help` | Print help information | N/A |
| `-V, --version` | Print version information | N/A |

Full attribution options:
- `ru` - Russian (default)
- `zh` - Chinese
- `ko` - Korean
- `de` - German
- `fr` - French
- `ar` - Arabic
- `fa` - Persian (Iran)
- `random` - Random attribution

## Plugin System

PhantomKeystroke supports integration with various C2 frameworks through its plugin system:

- **Null Plugin**: Standalone terminal mode for direct interaction
- **Cobalt Strike Plugin**: Integration with Cobalt Strike's External C2 interface
- **Sliver Plugin**: Integration with Sliver C2 via WebSocket connection
- **Mythic Plugin**: Integration with Mythic C2 REST API
- **Custom Plugins**: Load your own plugins as dynamic libraries (.so)

### Plugin Usage Examples

```bash
# Proxy commands through Cobalt Strike with Russian attribution
./target/release/phantom_keystroke --attribute --plugin=cobaltstrike --attribution ru

# Integrate with Sliver C2 using Chinese attribution
./target/release/phantom_keystroke --random --plugin=sliver --attribution zh

# Use Mythic C2 with random attribution patterns
./target/release/phantom_keystroke --attribute --plugin=mythic --attribution random
```

## Use Cases

- **Red Team Exercises**: Simulate a foreign operator to test your blue team's attribution skills
- **Forensic Research**: Study how subtle markers affect incident response analysis
- **C2 Enhancement**: Layer deception over Sliver or Mythic operations for added OPSEC
- **Attribution Research**: Understand the techniques used in false flag operations
- **Educational Purposes**: Learn about digital forensics and attribution challenges

## Configuration File

For persistent settings, create a `config.toml` file:

```toml
[mode]
type = 2  # 1=Random, 2=Attribute

[attribute]
country = "RU"
language = "ru"
timezone = "+3"

[plugin]
name = "null"  # null, cobaltstrike, sliver, mythic, custom
```

## Technical Implementation

PhantomKeystroke operates by:

1. **Fingerprint Injection**: Adding subtle region-specific artifacts to commands without affecting functionality
2. **Keyboard Emulation**: Simulating realistic typing patterns for human-like operation
3. **Command Transformation**: Preserving full command functionality while adding attribution markers
4. **Plugin Coordination**: Integrating with C2 frameworks seamlessly

## Security Notes

- No root privileges required, reducing system exposure
- OPSEC validation to catch potential attribution breaches
- Avoid persistent storage of sensitive data
- Run in a disposable VM for enhanced anonymity
- Ensure plugins don't leak attribution data back to C2 servers—verify with `--verbose`
- Running outside a VM may leave host-specific artifacts, undermining the deception
- Sophisticated analysts might detect consistent patterns—use random mode to mitigate

## License

PhantomKeystroke is released under the **MIT License**. See [LICENSE](https://github.com/JAYGLXR/phantomkeystroke/blob/main/LICENSE) for details.

## Contributing

- Fork the repository at [https://github.com/JAYGLXR/phantomkeystroke](https://github.com/JAYGLXR/phantomkeystroke)
- Submit issues or pull requests for enhancements (e.g., new fingerprinting techniques, country profiles)
- Follow Rust coding standards and include tests

## Disclaimer

PhantomKeystroke is an educational tool focused on attribution techniques. The maintainers do not endorse malicious use. Use responsibly and legally. This tool is designed for security research, red team exercises, and understanding attribution challenges in cybersecurity. Users are responsible for ensuring compliance with applicable laws and regulations in their jurisdiction.

## Future Enhancements (Coming Soon)

PhantomKeystroke is under active development with several exciting features on the roadmap:

### Advanced C2 Framework Integrations
- **Havoc C2 Integration**: Native support for the Havoc framework with attribution-aware payload delivery
- **Brute Ratel Integration**: Seamless operation with Brute Ratel C4 with fingerprint preservation
- **Covenant Integration**: Support for .NET-based C2 operations with attribution markers
- **Custom Protocol Handlers**: Develop your own protocol handlers for proprietary C2 frameworks

### Time and Latency-Based Deception
- **Working Hours Emulation**: Automatically restrict operations to working hours of the target attribution region
- **Network Latency Simulation**: Add realistic network delays consistent with the attributed country's infrastructure
- **Holiday/Weekend Awareness**: Built-in calendar to avoid operations during national holidays of the attributed country
- **Typing Speed Variations**: Region-specific typing patterns based on keyboard layout and language proficiency

### Enhanced Attribution Techniques
- **Browser Fingerprinting**: Leave browser artifacts consistent with the target region
- **File Metadata Manipulation**: Automatically adjust document metadata to match regional patterns
- **Language-Specific Comments**: Insert regionally appropriate comments in scripts and code
- **Error Message Localization**: Generate error messages with regional language patterns

### Inspiration and Research

The development of PhantomKeystroke's attribution deception capabilities was partly inspired by research into nation-state TTPs, including the fascinating analysis of [NSA (Equation Group) TTPs from China's perspective](https://www.inversecos.com/2025/02/an-inside-look-at-nsa-equation-group.html). This research highlights how operational patterns like working hours, keyboard inputs, and human errors can be used for attribution - patterns that PhantomKeystroke aims to emulate and control.

Our roadmap is focused on implementing the most sophisticated techniques observed in the wild while making them accessible for educational and defensive research purposes.
