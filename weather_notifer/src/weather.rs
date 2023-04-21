use chrono::{DateTime, Datelike, FixedOffset};
use serde::Deserialize;
use std::fmt::{self, Display};
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct WeatherReport {
    pub publishingOffice: String,
    pub reportDatetime: String,
    pub timeSeries: Vec<TimeSeries>,
}

#[derive(Debug, Clone)]
pub struct TimeSpecificData {
    pub time: DateTime<FixedOffset>,
    pub area_name: String,
    pub weathers: Vec<String>,
    pub temps: Vec<String>,
}

impl WeatherReport {
    // 曜日を返すプライベートメソッド（モック実装）
    fn _day_of_week(&self, date: DateTime<FixedOffset>) -> &'static str {
        match date.weekday() {
            chrono::Weekday::Mon => "Mon",
            chrono::Weekday::Tue => "Tue",
            chrono::Weekday::Wed => "Wed",
            chrono::Weekday::Thu => "Thu",
            chrono::Weekday::Fri => "Fri",
            chrono::Weekday::Sat => "Sat",
            chrono::Weekday::Sun => "Sun",
        }
    }

    pub fn display_weather_and_temperature(&self) -> String {
        let mut output = String::new();
        output.push_str("Time-specific Weather:\n");
        output.push_str(self.display_time_specific_data("weather").as_str());

        output.push_str("\nTime-specific Temperature:\n");
        output.push_str(self.display_time_specific_data("temperature").as_str());
        output
    }

    fn display_time_specific_data(&self, data_type: &str) -> String {
        let data = self.extract_time_specific_data();
        let mut output = String::new();
        match data_type {
            "weather" => {
                let content = data
                    .iter()
                    .filter(|area| !area.weathers.is_empty())
                    .map(|area| {
                        format!(
                            "| {:<15} | {:<12} | {:<15} |",
                            area.time.format("%m/%d %H:%M %a"),
                            area.area_name,
                            area.weathers.join(" / ")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                if !content.is_empty() {
                    let header = "| Time            | Area         | Weather         |\n| --------------- | ------------ | --------------- |";
                    output.push_str(header);
                    output.push_str("\n");
                    output.push_str(content.as_str());
                }
            }
            "temperature" => {
                let content = data
                    .iter()
                    .filter(|area| !area.temps.is_empty())
                    .map(|area| {
                        format!(
                            "| {:<15} | {:<12} | {:<16} |",
                            area.time.format("%m/%d %H:%M %a"),
                            area.area_name,
                            area.temps.join(" / ")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                if !content.is_empty() {
                    let header = "| Time            | Area         | Temperature (℃) |\n| --------------- | ------------ | ---------------- |";
                    output.push_str(header);
                    output.push_str("\n");
                    output.push_str(content.as_str());
                }
            }
            _ => {}
        }
        output.push_str("\n");
        output
    }

    fn extract_time_specific_data(&self) -> Vec<TimeSpecificData> {
        let mut data = Vec::new();
        #[allow(non_snake_case)]
        for timeSeries in &self.timeSeries {
            let time = timeSeries.time_defines[0]
                .parse::<DateTime<FixedOffset>>()
                .unwrap();
            for area in &timeSeries.areas {
                let area_name = area.area.name.clone();
                let weathers = area.weathers.clone();
                let temps = area.temps.clone();
                data.push(TimeSpecificData {
                    time,
                    area_name,
                    weathers,
                    temps,
                });
            }
        }
        data
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TimeSeries {
    #[serde(rename = "timeDefines")]
    pub time_defines: Vec<String>,
    pub areas: Vec<Area>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Area {
    pub area: AreaDetail,
    #[serde(default)]
    pub weather_codes: Vec<String>,
    #[serde(default)]
    pub weathers: Vec<String>,
    #[serde(default)]
    pub winds: Vec<String>,
    #[serde(default)]
    pub waves: Vec<String>,
    #[serde(default)]
    pub pops: Vec<String>,
    #[serde(default)]
    pub temps: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AreaDetail {
    pub name: String,
    pub code: String,
}

impl Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Publishing Office: {}", self.publishingOffice)?;
        writeln!(f, "Report Datetime: {}", self.reportDatetime)?;
        writeln!(f, "Time Series:")?;
        for (i, timeSeries) in self.timeSeries.iter().enumerate() {
            writeln!(f, "  [{}] Time Defines:", i)?;
            for time_define in &timeSeries.time_defines {
                writeln!(f, "    {}", time_define)?;
            }
            writeln!(f, "  [{}] Areas:", i)?;
            for area in &timeSeries.areas {
                writeln!(f, "    Area: {} ({})", area.area.name, area.area.code)?;
                if !area.weather_codes.is_empty() {
                    writeln!(f, "    Weather Codes: {:?}", area.weather_codes)?;
                }
                if !area.weathers.is_empty() {
                    let sanitized_weathers: Vec<String> = area
                        .weathers
                        .iter()
                        .map(|weather| sanitize_unicode_spaces(weather))
                        .collect();
                    writeln!(f, "    Weathers: {:?}", sanitized_weathers)?;
                }
                if !area.winds.is_empty() {
                    let sanitized_winds: Vec<String> = area
                        .winds
                        .iter()
                        .map(|wind| sanitize_unicode_spaces(wind))
                        .collect();
                    writeln!(f, "    Winds: {:?}", sanitized_winds)?;
                }
                if !area.waves.is_empty() {
                    let sanitized_waves: Vec<String> = area
                        .waves
                        .iter()
                        .map(|wave| sanitize_unicode_spaces(wave))
                        .collect();
                    writeln!(f, "    Waves: {:?}", sanitized_waves)?;
                }
                if !area.pops.is_empty() {
                    writeln!(f, "    Pops: {:?}", area.pops)?;
                }
                if !area.temps.is_empty() {
                    writeln!(f, "    Temps: {:?}", area.temps)?;
                }
            }
        }
        Ok(())
    }
}

fn sanitize_unicode_spaces(input: &str) -> String {
    input.replace('\u{3000}', " ")
}

// ApiResponseをVec<WeatherReport>の型エイリアスとして定義
pub type ApiResponse = Vec<WeatherReport>;
