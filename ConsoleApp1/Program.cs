using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System.Security.Cryptography.X509Certificates;
using System.Net;
using System.Net.Security;
using System.IO;

using System.Text.Json;
using System.Text.Json.Serialization;

namespace ConsoleApp1
{
    internal class Program
    {
        public partial class Weather
        {
            public Data Data { get; set; }
            public string Message { get; set; }
            public long Status { get; set; }
        }

        public partial class Data
        {
            public Observe Observe { get; set; }
        }

        public partial class Observe
        {
            public long Degree { get; set; }
            public long Humidity { get; set; }
            public long Precipitation { get; set; }
            public long Pressure { get; set; }
            public string UpdateTime { get; set; }
            public string Weather { get; set; }
            public string WeatherCode { get; set; }
            public string WeatherShort { get; set; }
            public long WindDirection { get; set; }
            public long WindPower { get; set; }
        }
        public static bool CheckValidationResult(object sender,X509Certificate certificate,X509Chain chain,SslPolicyErrors errors)
        {
            return true;
        }
        public static string GetInfo(string url)
        {
            ServicePointManager.ServerCertificateValidationCallback = new System.Net.Security.RemoteCertificateValidationCallback(CheckValidationResult);
            string strBuff = "";
            Uri httpURL=new Uri(url);
            HttpWebRequest httpReq=(HttpWebRequest)WebRequest.Create(httpURL);
            HttpWebResponse httpResp=(HttpWebResponse)httpReq.GetResponse();
            Stream respStream=httpResp.GetResponseStream();
            StreamReader respStreamReader = new StreamReader(respStream, Encoding.UTF8);
            strBuff = respStreamReader.ReadToEnd();
            return strBuff;
        }
        static void Main(string[] args)
        {
            Console.InputEncoding = Encoding.Unicode;
            string province, city;
            Console.Write("请输入省份（中文）：");
            province = Console.ReadLine();
            Console.Write("请输入城市（中文）：");
            city = Console.ReadLine();
            Console.WriteLine("省份："+province+";"+"城市："+city);
            string url = "https://wis.qq.com/weather/common?source=pc&city=" + city + "&province=" + province + "&weather_type=observe";
            string weatherInfo = GetInfo(url);
            //Console.WriteLine(weatherInfo);
            JObject weather_JObject=(JObject)JsonConvert.DeserializeObject(weatherInfo);
            //string data = weather_JObject["data"]["observe"].ToString();
            //Console.WriteLine(data);
            string degree = weather_JObject["data"]["observe"]["degree"].ToString();
            Console.WriteLine($"degree:{degree}");
            string weather = weather_JObject["data"]["observe"]["weather"].ToString();
            Console.WriteLine($"weather:{weather}");
            //Weather weather=System.Text.Json.JsonSerializer.Deserialize<Weather>(weatherInfo);
            //Console.WriteLine(weather.Message);
            Console.ReadKey();
        }
    }
}
/*
{
    "data":
    {
        "observe":
        {
            "degree":"8",
            "humidity":"81",
            "precipitation":"0",
            "pressure":"1024",
            "update_time":"202303180830",
            "weather":"多云",
            "weather_code":"01",
            "weather_short":"多云",
            "wind_direction":"2",
            "wind_power":"1"
        }
        
    },
    "message":"OK",
    "status":200
}
 */
