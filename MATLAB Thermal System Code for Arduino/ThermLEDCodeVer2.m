
% bring in arduino object
%           port#   ardType   library      import DHT11 lib
a = arduino('COM8', 'Micro', 'Libraries', 'Adafruit/DHTxx'); % this is bringing in the arduino

% bring in the Humidity sensor
humiditySensor = addon(a, 'Adafruit/DHTxx','D6','DHT11'); % id sensor type

v = readVoltage(a,'A1'); % read in the voltage from I/O pin that thermistor is connected with
tempC = (v - 0.5)*100; % turn voltage to celsius 
humidPercent = readHumidity(humiditySensor); % read humidity as a percent

% TEMPERATURE VARIABLES
tooHot = 60; % degrees CELSIUS

% below is the logic for when the LED will light to signal that temps are
if tempC > tooHot
    while tooHot > tempC
    writeDigitalPin(a,'D7',1); % LED = ON
    pause(1); % seconds
    writeDigitalPin(a,'D7',0); % LED = ON
    pause(1); % seconds
    end
end





