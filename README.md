# wthr — weather from the terminal

A minimal CLI weather tool. One binary, three dependencies, no API key.

```
wthr London       → ☁️  28°C, feels 21°C, wind 15 km/h WNW
wthr forecast Tokyo → 3 days: high/low, condition
```

Powered by [wttr.in](https://wttr.in) — free, no signup, no rate limits.

## Install

```bash
cargo install --path .
```

Or build standalone:

```bash
cargo build --release
cp target/release/wthr ~/.local/bin/
```

## Usage

```
wthr <city>              Current weather
wthr forecast <city>     3-day forecast
wthr help                This help
```

City names with spaces work naturally:

```bash
wthr London
wthr forecast Paris
wthr "New York"
wthr 90210
```

## Output

Current weather:

```
 Walworth
 Partly Cloudy

   28°C  (feels 21°C)
   Wind: 15 km/h WNW   Humidity: 33%
```

Forecast:

```
 Shikinejima — 3-day forecast

   2026-07-28   27°C / 25°C   Partly Cloudy
   2026-07-29   28°C / 27°C   Patchy rain nearby
   2026-07-30   27°C / 26°C   Partly Cloudy
```

## Build

Requires Rust 1.75+.

```bash
cargo build --release
# target/release/wthr — 85 KB stripped
```

## License

MIT
