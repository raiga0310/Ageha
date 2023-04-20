use serde::Deserialize;
use std::fmt::{self, Display};

#[derive(Debug, Deserialize, Clone)]
pub struct WeatherReport {
    pub publishingOffice: String,
    pub reportDatetime: String,
    pub timeSeries: Vec<TimeSeries>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TimeSeries {
    #[serde(rename = "timeDefines")]
    pub timeDefines: Vec<String>,
    pub areas: Vec<Area>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Area {
    pub area: AreaDetail,
    #[serde(default)]
    pub weatherCodes: Vec<String>,
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
        for (i, time_series) in self.timeSeries.iter().enumerate() {
            writeln!(f, "  [{}] Time Defines:", i)?;
            for time_define in &time_series.timeDefines {
                writeln!(f, "    {}", time_define)?;
            }
            writeln!(f, "  [{}] Areas:", i)?;
            for area in &time_series.areas {
                writeln!(f, "    Area: {} ({})", area.area.name, area.area.code)?;
                if !area.weatherCodes.is_empty() {
                    writeln!(f, "    Weather Codes: {:?}", area.weatherCodes)?;
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
