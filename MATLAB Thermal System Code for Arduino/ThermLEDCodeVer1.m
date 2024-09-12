% LED and thermistor intro code
ledPin = 'D13'; % use a digital pin and the respective number, arbritrary

% bring in arduino object
port = 'COM5'; % this is the port that the arduino USB is plugged into, currently arbritratry
board = 'Due'; % this will need to be updated

a = arduino(port, board); % this is bringing in the arduino
temperature = readTemperature(

% We can read the output voltage, convert it to Celsius and convert the
% result to Farenheit as follows:
v = readVoltage(a,'A0'); % read in the voltage
TempC = (v - 0.5)*100; % turn voltage to celsius 


