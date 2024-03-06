import requests

url = "https://wis.qq.com/weather/common?source=pc&city={city}&province={province}&weather_type=observe"

payload = {}
headers = {}

response = requests.request("GET", url, headers=headers, data=payload)

print(response.text)