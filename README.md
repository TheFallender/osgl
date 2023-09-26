# OpenSSL Game Launcher (OSGL)

## Overview

The OpenSSL Game Launcher (OSGL) is a utility designed to set specific environment variables required to fix OpenSSL issues in games, these are known issues for [all Intel 10th+ generation processors](https://r2northstar.gitbook.io/r2northstar-wiki/installing-northstar/troubleshooting#intel).

Unfortunately, this requires games to be updated by their developers, and some publishers/developers so... yeah we know how that goes. 

This is a generic launcher written in Rust that can be used for any game suffering from this issue. I made this to be as lightweight and simple as possible, so it shouldn't trigger any antivirus software.

## How it Works

The launcher works by setting the `OPENSSL_ia32cap` environment variable and then launching the original game executable.

## Can I be banned on X game for using this?

While this doesn't modify anything from the game and just sets an environment variable for an executable, I can't guarantee that you won't be banned for using this.

## Why shouldn't I just set the environment variable myself?

You absolutely can, but doing that will set the environment variable for your entire system, which is not ideal as it may cause issues with other games or software or performance loss.

This launcher will only set the environment variable for the game you want to launch.

## Is there any other way to fix this?

Yes, you can set the Steam launch options for the game to include the environment variable, but this will only work for Steam games that launch from Steam, so Titanfall 2 wouldn't work from Steam.

## How to Use

1. Rename the original game `.exe` to `game_original.exe`.
2. Rename the launcher to `game.exe`.
3. Launch the game as you normally would.

#### Example

For example, I mainly made this thinking of `Titanfall 2`, so the steps would be:
1. Rename `Titanfall2.exe` to `Titanfall2_original.exe`.
2. Rename `osgl.exe` to `Titanfall2.exe`.
3. Profit.

## Compilation

This utility is written in Rust. To compile, you can use:

```bash
cargo build --release
```

## Dependencies

- Rust
- Cargo

## License

This project is open-source and available under the MIT License.

