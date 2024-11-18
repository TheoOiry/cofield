#include <BLEDevice.h>
#include <BLEServer.h>
#include <BLEUtils.h>
#include <BLE2902.h>
#include <BLE2901.h>

BLEServer *pServer = NULL;
BLECharacteristic *pCharacteristic = NULL;
BLE2901 *descriptor_2901 = NULL;

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

  BLEDevice::init(BLE_DEVICE_NAME);

  pServer = BLEDevice::createServer();
  pServer->setCallbacks(new MyServerCallbacks());

  BLEService *pService = pServer->createService(SERVICE_UUID);

  pCharacteristic = pService->createCharacteristic(
    CHARACTERISTIC_UUID,
    BLECharacteristic::PROPERTY_READ | BLECharacteristic::PROPERTY_WRITE | BLECharacteristic::PROPERTY_NOTIFY
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

  for (int i = 0; i < 5; i++) {
    pinMode(A0 + i, INPUT);
  }

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

  for (int i = 0; i < 5; i++) {
    uint16_t sensorValue = analogRead(A0 + i);

    buffer[i * 2] = (uint8_t)(sensorValue & 0xff);
    buffer[i * 2 + 1] = (uint8_t)(sensorValue >> 8);
  }

  unsigned long timeStamp = millis();
  buffer[10] = (uint8_t)(timeStamp & 0xff);         
  buffer[11] = (uint8_t)((timeStamp >> 8) & 0xff);  
  buffer[12] = (uint8_t)((timeStamp >> 16) & 0xff); 
  buffer[13] = (uint8_t)((timeStamp >> 24) & 0xff);

  return buffer;
}