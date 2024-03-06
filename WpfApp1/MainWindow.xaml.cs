using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net.Security;
using System.Net;
using System.Security.Cryptography.X509Certificates;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using System.Xml.Linq;

using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace WpfApp1
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
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
        public static bool CheckValidationResult(object sender, X509Certificate certificate, X509Chain chain, SslPolicyErrors errors)
        {
            return true;
        }
        public static string GetInfo(string url)
        {
            ServicePointManager.ServerCertificateValidationCallback = new System.Net.Security.RemoteCertificateValidationCallback(CheckValidationResult);
            string strBuff = "";
            Uri httpURL = new Uri(url);
            HttpWebRequest httpReq = (HttpWebRequest)WebRequest.Create(httpURL);
            HttpWebResponse httpResp = (HttpWebResponse)httpReq.GetResponse();
            Stream respStream = httpResp.GetResponseStream();
            StreamReader respStreamReader = new StreamReader(respStream, Encoding.UTF8);
            strBuff = respStreamReader.ReadToEnd();
            return strBuff;
        }
        public MainWindow()
        {
            InitializeComponent();
        }

        private void OnClick(object sender, RoutedEventArgs e)
        {
            string province, city;
            province=TextBox_Porvince.Text;
            city=TextBox_City.Text;
            string url = "https://wis.qq.com/weather/common?source=pc&city=" + city + "&province=" + province + "&weather_type=observe";
            string weatherInfo = GetInfo(url);
            JObject weather_JObject = (JObject)JsonConvert.DeserializeObject(weatherInfo);
            string degree = weather_JObject["data"]["observe"]["degree"].ToString();
            string weather = weather_JObject["data"]["observe"]["weather"].ToString();
            textBlock.FontSize= 18;
            textBlock.Inlines.Add(new Bold(new Run("Province(省份):")));
            textBlock.Inlines.Add(new Italic(new Run(province+"\n")));
            textBlock.Inlines.Add(new Bold(new Run("City(城市):")));
            textBlock.Inlines.Add(new Italic(new Run(city+"\n")));
            textBlock.Inlines.Add(new Bold(new Run("Degree(温度):")));
            textBlock.Inlines.Add(new Italic(new Run(degree + "\n")));
            textBlock.Inlines.Add(new Bold(new Run("Weather(天气):")));
            textBlock.Inlines.Add(new Italic(new Run(weather + "\n")));
        }
    }
}
