/*********
  Complete project details at https://RandomNerdTutorials.com/raspberry-pi-pico-ds18b20-arduino/
  Example adapted from the microDS18B20 library examples folder - microSD18B20 library: https://github.com/GyverLibs/microDS18B20
*********/

#include <microDS18B20.h>
#include <WiFi.h>
#include <ArduinoHttpClient.h>
#include "ssid.h"


// using GPIO 22, change accordingly
MicroDS18B20<22> sensor;
const char* ssid = STASSID;
const char* password = STAPSK;

WiFiMulti multi;
WiFiClient wifiClient;
HttpClient httpClient = HttpClient(wifiClient, "192.168.1.112", 9090);

void setup() {
  Serial.begin(115200);

  //connect to wifi
  Serial.println();
  Serial.println();
  Serial.print("Connecting to ");
  Serial.println(ssid);

  multi.addAP(ssid, password);

  if (multi.run() != WL_CONNECTED) {
    Serial.println("Unable to connect to network, rebooting in 10 seconds...");
    delay(10000);
    rp2040.reboot();
  }

  Serial.println("");
  Serial.println("WiFi connected");
  Serial.println("IP address: ");
  Serial.println(WiFi.localIP());
}

void loop() {
  static bool wait = false;
  float temp = 0;

  sensor.requestTemp();
  temp = sensor.getTemp();


  String contentType = "application/json";
  String postData = "";
  String url = "/temp?room=mada&temp=" + String(temp);
  Serial.println("making POST request:" + url);
  httpClient.post(url, contentType, postData);

  int statusCode = httpClient.responseStatusCode();
  String response = httpClient.responseBody();
  Serial.print("Status code: ");
  Serial.println(statusCode);
  Serial.print("Response: ");
  Serial.println(response);
  
  if (wait) {
    delay(60000);  //  don't flood remote service
  }
  wait = true;
}
