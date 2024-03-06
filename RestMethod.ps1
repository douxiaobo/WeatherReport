$response = Invoke-RestMethod 'https://wis.qq.com/weather/common?source=pc&city={city}&province={province}&weather_type=observe' -Method 'GET' -Headers $headers
$response | ConvertTo-Json