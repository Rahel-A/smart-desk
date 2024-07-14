mod smart_desk;

use smart_desk::desk_actions::SmartDesk;
use smart_desk::hardware_error::HardwareError;

fn main() -> Result<(), HardwareError> {
    env_logger::init();

    const GPIO_RELAY_DOWN: u8 = 22; // pin#15
    const GPIO_RELAY_UP: u8 = 5; // pin#29

    let data = smart_desk::settings::PersistentData::get_persistent_data().expect("no persistent data");
    SmartDesk::new(
        GPIO_RELAY_UP,
        GPIO_RELAY_DOWN,
        data.upper_height(),
        data.lower_height(),
    )?
    .run()
}
