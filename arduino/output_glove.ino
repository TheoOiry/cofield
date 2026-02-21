#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>
#include <BLE2902.h>
#include <BLE2901.h>
#include <Wire.h>
#include <ADS1X15.h>

BLEServer *pServer = NULL;
BLECharacteristic *pCharacteristic = NULL;
BLE2901 *descriptor_2901 = NULL;

ADS1115 ads_48(0x48);
ADS1115 ads_49(0x49);

const int PERIOD = 20;


bool deviceConnected = false;
bool oldDeviceConnected = false;

unsigned long startMillis = NULL;
unsigned long timeNow = 0;

#define SERVICE_UUID "f5874094-9074-4bb6-9257-f3593d73d836"
#define CHARACTERISTIC_UUID "a81ed63c-cf54-4742-a27a-f398228acd90"
#define BLE_DEVICE_NAME "FlexSensorGlove"

class MyServerCallbacks : public BLEServerCallbacks {
  void onConnect(BLEServer *pServer) {
    deviceConnected = true;
  };

  void onDisconnect(BLEServer *pServer) {
    deviceConnected = false;
    startMillis = NULL;
  }
};

void setup() {
  Serial.begin(115200);

  Wire.begin();

  if (!ads_48.begin()) {
    Serial.println("ADS1115 0x48 not found!");
  }

  if (!ads_49.begin()) {
    Serial.println("ADS1115 0x49 not found!");
  }

  ads_48.setGain(4);
  ads_48.setDataRate(4);
  ads_48.setMode(1);

  ads_49.setGain(4);
  ads_49.setDataRate(4);
  ads_49.setMode(1);

  BLEDevice::init(BLE_DEVICE_NAME);

  pServer = BLEDevice::createServer();
  pServer->setCallbacks(new MyServerCallbacks());

  BLEService *pService = pServer->createService(SERVICE_UUID);

  pCharacteristic = pService->createCharacteristic(
    CHARACTERISTIC_UUID,
    BLECharacteristic::PROPERTY_READ | BLECharacteristic::PROPERTY_WRITE | BLECharacteristic::PROPERTY_NOTIFY | BLECharacteristic::PROPERTY_INDICATE
  );

  pCharacteristic->addDescriptor(new BLE2902());

  descriptor_2901 = new BLE2901();
  descriptor_2901->setDescription("Notifications for flex sensors");
  descriptor_2901->setAccessPermissions(ESP_GATT_PERM_READ);
  pCharacteristic->addDescriptor(descriptor_2901);

  pService->start();

  BLEAdvertising *pAdvertising = BLEDevice::getAdvertising();
  pAdvertising->addServiceUUID(SERVICE_UUID);
  pAdvertising->setScanResponse(false);
  pAdvertising->setMinPreferred(0x0);
  BLEDevice::startAdvertising();

  Serial.println("Waiting a client connection to notify...");
}

void loop() {
  if (deviceConnected && millis() >= timeNow + PERIOD) {
    timeNow += PERIOD;

    if (startMillis == NULL) {
      startMillis = millis();
    }

    pCharacteristic->setValue(readSensors(), 14);
    pCharacteristic->notify();
  }
    
  if (!deviceConnected && oldDeviceConnected) {
    delay(500);                   // give the bluetooth stack the chance to get things ready
    pServer->startAdvertising();  // restart advertising
    Serial.println("start advertising");
    oldDeviceConnected = deviceConnected;
  }
  
  if (deviceConnected && !oldDeviceConnected) {  
    oldDeviceConnected = deviceConnected;
  }
}

uint8_t* readSensors() {
  static uint8_t buffer[14];

  int16_t values[5];

  values[0] = ads_48.readADC_Differential_2_3();
  values[1] = ads_48.readADC_Differential_1_3();
  values[2] = ads_48.readADC_Differential_0_3();

  values[3] = ads_49.readADC_Differential_2_3();
  values[4] = ads_49.readADC_Differential_1_3();

  for (int i = 0; i < 5; i++) {
    buffer[i * 2]     = (uint8_t)(values[i] & 0xFF);
    buffer[i * 2 + 1] = (uint8_t)((values[i] >> 8) & 0xFF);
  }

  unsigned long timeStamp = millis();
  buffer[10] = (uint8_t)(timeStamp & 0xff);
  buffer[11] = (uint8_t)((timeStamp >> 8) & 0xff);
  buffer[12] = (uint8_t)((timeStamp >> 16) & 0xff);
  buffer[13] = (uint8_t)((timeStamp >> 24) & 0xff);

  return buffer;
}