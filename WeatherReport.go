// weatherreaport.go

// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse and unparse this JSON data, add this code to your project and do:
//
//    weatherReaport, err := UnmarshalWeatherReaport(bytes)
//    bytes, err = weatherReaport.Marshal()

package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

func UnmarshalWeatherReaport(data []byte) (WeatherReaport, error) {
	var r WeatherReaport
	err := json.Unmarshal(data, &r)
	if err != nil {
		fmt.Printf("Unmarshal err=%v\n", err)
	}
	var degree1 = string(*r.Data.Observe.Degree)
	fmt.Print("温度：")
	fmt.Println(degree1)
	var weather1 = string(*r.Data.Observe.Weather)
	fmt.Print("天气：")
	fmt.Println(weather1)
	return r, err
}

func (r *WeatherReaport) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type WeatherReaport struct {
	Data    *Data   `json:"data,omitempty"`
	Message *string `json:"message,omitempty"`
	Status  *int64  `json:"status,omitempty"`
}

/*
{
	"data":{
		"observe":{
			"degree":"26",
			"humidity":"80",
			"precipitation":"0",
			"pressure":"1005",
			"update_time":"202209151900",
			"weather":"多云",
			"weather_code":"01",
			"weather_short":"多云",
			"wind_direction":"1",
			"wind_power":"1"
			}
		},
	"message":"OK",
	"status":200
}
*/

func main() {
	var province, city string
	fmt.Printf("请输入省份（中文）：\n\t")
	fmt.Scan(&province)
	fmt.Printf("请输入城市（中文）：\n\t")
	fmt.Scan(&city)
	fmt.Printf(province + "\t" + city + "\n")
	fmt.Println()

	url := "https://wis.qq.com/weather/common?source=pc&city=" + city + "&province=" + province + "&weather_type=observe"
	//url := "https://wis.qq.com/weather/common?source=pc&city=宁波&province=浙江&weather_type=observe"

	response, err := http.Get(url)
	if err != nil {
		log.Println(err)
		return
	}
	defer response.Body.Close()
	//	fmt.Println()

	result, err := ioutil.ReadAll(response.Body)
	if err != nil {
		log.Println(err)
		return
	}
	//fmt.Println(string(result))
	UnmarshalWeatherReaport(result)

}
