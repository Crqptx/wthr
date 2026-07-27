use std::env;
use std::process;

use serde::Deserialize;

// ── Serde models matching wttr.in JSON ──

#[derive(Debug, Deserialize)]
struct WttrResponse {
    current_condition: Vec<CurrentCondition>,
    weather: Vec<ForecastDay>,
    nearest_area: Vec<Area>,
}

#[derive(Debug, Deserialize)]
struct CurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    humidity: String,
    #[serde(rename = "windspeedKmph")]
    windspeed_kmph: String,
    #[serde(rename = "winddir16Point")]
    winddir16_point: String,
    #[serde(rename = "FeelsLikeC")]
    feels_like: Option<String>,
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<TextValue>,
}

#[derive(Debug, Deserialize)]
struct Area {
    #[serde(rename = "areaName")]
    area_name: Vec<TextValue>,
}

#[derive(Debug, Deserialize)]
struct ForecastDay {
    date: String,
    #[serde(rename = "maxtempC")]
    maxtemp_c: String,
    #[serde(rename = "mintempC")]
    mintemp_c: String,
    hourly: Vec<Hourly>,
}

#[derive(Debug, Deserialize)]
struct Hourly {
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<TextValue>,
}

#[derive(Debug, Deserialize)]
struct TextValue {
    value: String,
}

// ── CLI command ──

enum Command {
    Current { city: String },
    Forecast { city: String },
    Help,
}

fn parse_args() -> Command {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => Command::Help,
        2 if args[1] == "help" => Command::Help,
        2 => Command::Current { city: args[1].clone() },
        3 if args[1] == "forecast" => Command::Forecast {
            city: args[2].clone(),
        },
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}

// ── HTTP fetch ──

fn fetch_weather(city: &str) -> Result<WttrResponse, String> {
    let url = format!("https://wttr.in/{}?format=j1", city.replace(' ', "%20"));
    let resp = reqwest::blocking::get(&url).map_err(|e| format!("Network error: {}", e))?;
    let status = resp.status();
    let body = resp.text().map_err(|e| format!("Read error: {}", e))?;

    if !status.is_success() {
        return Err(format!("HTTP {} from wttr.in", status));
    }

    serde_json::from_str(&body).map_err(|e| format!("Parse error: {}", e))
}

// ── Render ──

const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn render_current(w: &WttrResponse) {
    let area = w.nearest_area.first().and_then(|a| a.area_name.first().map(|n| &n.value));
    let city = area.map(|s| s.as_str()).unwrap_or("Unknown");

    if let Some(cc) = w.current_condition.first() {
        let desc = cc.weather_desc.first().map(|d| d.value.as_str()).unwrap_or("");
        let temp = &cc.temp_c;
        let feels = cc.feels_like.as_deref().unwrap_or(temp);
        let wind = &cc.windspeed_kmph;
        let dir = &cc.winddir16_point;
        let humidity = &cc.humidity;

        println!();
        println!(" {BOLD}{CYAN}{city}{RESET}");
        println!(" {desc}");
        println!();
        println!("   {BOLD}{temp}°C{RESET}  (feels {feels}°C)");
        println!("   {DIM}Wind:{RESET} {wind} km/h {dir}   {DIM}Humidity:{RESET} {humidity}%");
        println!();
    } else {
        println!("City not found: {}", city);
    }
}

fn render_forecast(w: &WttrResponse) {
    let area = w.nearest_area.first().and_then(|a| a.area_name.first().map(|n| &n.value));
    let city = area.map(|s| s.as_str()).unwrap_or("Unknown");

    if w.weather.is_empty() {
        println!();
        println!(" City not found: {}", city);
        println!();
        return;
    }

    println!();
    println!(" {BOLD}{CYAN}{city} — 3-day forecast{RESET}");
    println!();

    for day in &w.weather {
        let high = &day.maxtemp_c;
        let low = &day.mintemp_c;
        let desc = day
            .hourly
            .first()
            .and_then(|h| h.weather_desc.first().map(|d| d.value.as_str()))
            .unwrap_or("");

        let date = &day.date;

        println!("   {BOLD}{date}{RESET}   {YELLOW}{high}°C{RESET} / {low}°C   {GREEN}{desc}{RESET}");
    }
    println!();
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  wthr <city>              Current weather");
    eprintln!("  wthr forecast <city>     3-day forecast");
    eprintln!("  wthr help                This help");
}

// ── Main ──

fn main() {
    match parse_args() {
        Command::Current { city } => handle_weather(&city, false),
        Command::Forecast { city } => handle_weather(&city, true),
        Command::Help => {
            print_usage();
        }
    }
}

fn handle_weather(city: &str, forecast: bool) {
    match fetch_weather(city) {
        Ok(w) if forecast => render_forecast(&w),
        Ok(w) => render_current(&w),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_current_holds_city() {
        let cmd = Command::Current { city: "London".into() };
        match cmd {
            Command::Current { city } => assert_eq!(city, "London"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn command_forecast_holds_city() {
        let cmd = Command::Forecast { city: "Tokyo".into() };
        match cmd {
            Command::Forecast { city } => assert_eq!(city, "Tokyo"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn urlencodes_spaces() {
        let city = "New York";
        let url = format!("https://wttr.in/{}?format=j1", city.replace(' ', "%20"));
        assert_eq!(url, "https://wttr.in/New%20York?format=j1");
    }
}
