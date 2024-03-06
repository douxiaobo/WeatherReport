import requests

city=input('请输入省份，例如上海\n\t')
province=input('请输入城市,例如上海\n\t')

url = f"https://wis.qq.com/weather/common?source=pc&city=上海&province=上海&weather_type=observe"

payload = {}
headers = {}

response = requests.request("GET", url, headers=headers, data=payload)

print(response.text)