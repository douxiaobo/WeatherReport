// observe.go

package main

type Observe struct {
	Degree        *string `json:"degree,omitempty"`
	Humidity      *string `json:"humidity,omitempty"`
	Precipitation *string `json:"precipitation,omitempty"`
	Pressure      *string `json:"pressure,omitempty"`
	UpdateTime    *string `json:"update_time,omitempty"`
	Weather       *string `json:"weather,omitempty"`
	WeatherCode   *string `json:"weather_code,omitempty"`
	WeatherShort  *string `json:"weather_short,omitempty"`
	WindDirection *string `json:"wind_direction,omitempty"`
	WindPower     *string `json:"wind_power,omitempty"`
}
