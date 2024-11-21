use avr_hal_generic::hal::digital::v2::{InputPin, OutputPin};
use avr_hal_generic::port::Pin;
use core::time::Duration;
use arduino_hal::delay_us;

pub struct Dht11<T>
where
    T: InputPin + OutputPin,
{
    data_pin: T,
}

impl<T> Dht11<T>
where
    T: InputPin + OutputPin,
{
    pub fn new(data_pin: T) -> Self {
        Dht11 { data_pin }
    }

    pub fn read(&mut self) -> Result<(u8, u8), &'static str> {
        let mut humidity: u8 = 0;
        let mut temperature: u8 = 0;

        // Send start signal
        self.data_pin.set_low().map_err(|_| "Failed to pull pin low")?;
        delay_us(18_000); // 18 ms delay
        self.data_pin.set_high().map_err(|_| "Failed to pull pin high")?;
        delay_us(20); // 20 us delay

        // Switch to input mode
        // Assume `data_pin` is configured for bidirectional mode; implementation may vary by board.
        let mut data_pin = self.data_pin.into_floating_input();

        // Wait for the sensor response (LOW for 80us, HIGH for 80us)
        if !self.wait_for_pin_state(&data_pin, false, 80) {
            return Err("Sensor did not respond");
        }
        if !self.wait_for_pin_state(&data_pin, true, 80) {
            return Err("Sensor response was incomplete");
        }

        // Read 40 bits of data (5 bytes)
        let mut data: [u8; 5] = [0; 5];
        for byte in 0..5 {
            for bit in 0..8 {
                // Wait for the start of the bit signal (LOW)
                if !self.wait_for_pin_state(&data_pin, false, 50) {
                    return Err("Failed to detect bit start");
                }

                // Measure the duration of the HIGH signal
                let high_time = self.measure_high_time(&data_pin);

                // Determine if this is a 0 or 1
                if high_time > 30 {
                    data[byte] |= 1 << (7 - bit);
                }
            }
        }

        // Checksum validation
        let checksum = data[0].wrapping_add(data[1]).wrapping_add(data[2]).wrapping_add(data[3]);
        if checksum != data[4] {
            return Err("Checksum validation failed");
        }

        // Parse temperature and humidity
        humidity = data[0];
        temperature = data[2];

        Ok((humidity, temperature))
    }

    fn wait_for_pin_state(&self, pin: &T, state: bool, timeout_us: u16) -> bool {
        let mut elapsed = 0;
        while pin.is_high().unwrap_or(false) != state {
            delay_us(1);
            elapsed += 1;
            if elapsed >= timeout_us {
                return false;
            }
        }
        true
    }

    fn measure_high_time(&self, pin: &T) -> u16 {
        let mut elapsed = 0;
        while pin.is_high().unwrap_or(false) {
            delay_us(1);
            elapsed += 1;
            if elapsed >= 100 {
                break;
            }
        }
        elapsed
    }
}
