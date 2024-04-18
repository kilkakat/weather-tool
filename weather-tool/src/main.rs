use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    main: Main,
    wind: Wind,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
}

fn get_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
    let api_key = "bc82ec6c95374591a0f121826241704";
    let url = format!("https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", api_key, city);
    let response = reqwest::blocking::get(&url)?.json()?;
    Ok(response)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: weather <CITY>");
        return;
    }

    let city = &args[1];
    match get_weather(city) {
        Ok(weather) => {
            println!("Weather in {}: ", city);
            println!("Temperature: {}°C", weather.main.temp);
            println!("Feels like: {}°C", weather.main.feels_like);
            println!("Wind speed: {} m/s", weather.wind.speed);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}