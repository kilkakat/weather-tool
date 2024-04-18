use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    current: Current,
}

#[derive(Deserialize, Debug)]
struct Current {
    temp_c: f32,
    wind_kph: f32,
    feelslike_c: f32,
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
            println!("Temperature: {} °C", weather.current.temp_c);
            println!("Feels like: {} °C", weather.current.feelslike_c);
            println!("Wind speed: {} km/h", weather.current.wind_kph);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}