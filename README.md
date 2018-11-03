# i3gamma — adjust gamma per-window with i3

_i3gamma_ is a small application that integrates with the [i3][] window manager and changes the gamma correction value depending on the focused window. For example, in Sublime Text on XPS 13 9360, I prefer gamma 1.05, but in Team Fortress 2, the best gamma is 1.15.

_i3gamma_ is multi-monitor aware.

[i3]: https://i3wm.org

## Installation

Install [Rust][rustinst]. Then:

    cargo install i3gamma
    cp config.toml.example ~/.config/i3/gamma.toml

[rustinst]: https://www.rust-lang.org/en-US/install.html

## Configuration

_i3gamma_ uses a simple configuration file format that specifies the default gamma value for each monitor that needs to be adjusted, and the gamma value for any window that requires customization. Windows are identified by matching window titles exactly. Monitors are identified by their XRandR output names.

```toml
default-gamma = { eDP1 = 1.05 }

[[window]]
title = "Team Fortress 2 - OpenGL"
gamma = { eDP1 = 1.15 }
```

The path to the configuration file is provided as the command-line argument.

## Startup

_i3gamma_ does not daemonize or otherwise try to do anything special. It is suggested to run it via the desktop environment; in KDE, this can be configured by navigating to `System Settings` → `Startup and Shutdown` → `Autostart`, clicking `Add Program...`, and entering `~/.cargo/bin/i3gamma ~/.config/i3/gamma.toml`.

## License

_i3gamma_ is distributed under the terms of 0-clause BSD license.

See [LICENSE-0BSD](LICENSE-0BSD.txt) for details.
