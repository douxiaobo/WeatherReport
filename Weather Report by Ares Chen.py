# 请注意用 pip install requests进行安装
import requests
def get_weather(city: str, province: str) -> str:
    '''定义一个函数，用来根据省份，城市获取天气'''
    # 使用腾讯免费的天气服务，动态填入城市和省份信息，并且只获取当前的天气数据
    url = f'https://wis.qq.com/weather/common?source=pc&city={city}&province={province}&weather_type=observe'
    # 发起服务调用，请注意，不管有没有数据，它都会返回一个固定的json数据，里面包含了data属性，以及observe属性
    result = requests.get(url).json()['data']['observe']
    # 正常的返回结果是 {'degree': '23', 'humidity': '67', 'precipitation': '0', 'pressure': '1009', 'update_time': '202206072040', 'weather': '晴', 'weather_code': '00', 'weather_short': '晴', 'wind_direction': '5', 'wind_power': '1'}，所以这里读取出来温度和湿度信息
    output = f"温度:{result.get('degree','')}, 湿度:{result.get('humidity','')}"
    # 然后将结果返回
    return output
if __name__ == '__main__':
    province = input('请输入省份，例如上海\n\t')
    city = input('请输入城市,例如上海\n\t')
    print(f'查询天气预报为:{get_weather(city, province)}')