#include <BLEDevice.h>
#include <BLEUtils.h>
#include <BLEServer.h>

#define SERVICE_UUID "d8dcbaea-ca50-4758-bd9b-43667ff50a58"
#define CHARACTERISTIC_UUID "7f2697eb-0b07-40c7-ab91-a17be8b47650"
#define BLE_DEVICE_NAME "VibrationGlove"

class MyCallbacks : public BLECharacteristicCallbacks {
  void onWrite(BLECharacteristic *pCharacteristic) {
    uint8_t* data = pCharacteristic->getData();

    for (int i = 0; i < 5; i++) {
      analogWrite(A0 + i, data[i]);
    }
  }
};

void setup() {
  Serial.begin(115200);

  BLEDevice::init(BLE_DEVICE_NAME);
  BLEServer *pServer = BLEDevice::createServer();

  BLEService *pService = pServer->createService(SERVICE_UUID);

  BLECharacteristic *pCharacteristic =
    pService->createCharacteristic(CHARACTERISTIC_UUID, BLECharacteristic::PROPERTY_READ | BLECharacteristic::PROPERTY_WRITE);

  pCharacteristic->setCallbacks(new MyCallbacks());

  pCharacteristic->setValue("Vibrations listner");
  pService->start();

  BLEAdvertising *pAdvertising = pServer->getAdvertising();
  pAdvertising->start();

  for (int i = 0; i < 5; i++) {
    pinMode(A0 + i, OUTPUT);
  }
}

void loop() { }
