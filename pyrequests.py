import requests

def get_weather(city:str,province:str) ->str:
    url = f'https://wis.qq.com/weather/common?source=pc&city={city}&province={province}&weather_type=observe'
    payload = {}
    headers = {}
    response = requests.request("GET", url, headers=headers, data=payload).json()['data']['observe']
    output = f"天气：{response.get('weather','')},温度:{response.get('degree','')}, 湿度:{response.get('humidity','')}"
    return output

if __name__=='__main__':
    province = input('请输入省份，例如上海\n\t')
    city = input('请输入城市,例如上海\n\t')
    print(f'查询天气预报为:{get_weather(city, province)}')