use super::hal::{
    Distance, Interact, Interaction, MotorBuilder, MotorObject, MoveAction, RestAction,
    SmartDeskError, IO,
};
use super::hardware_error::HardwareError;
use super::motor::{Motor, MotorRelayPins, Moving, Resting};
use super::TableMotion;
use hc_sr04::HcSr04;
use std::thread;
use std::time::{Duration, Instant};

type HAL = IO<Motor<Resting>, Motor<Moving>, HcSr04>;
pub struct SmartDesk {
    hal: HAL,
    standing_height: u32,
    resting_height: u32,
}

impl SmartDesk {
    pub fn new(
        gpio_relay_up_pin: u8,
        gpio_relay_down_pin: u8,
        standing_height: u32,
        resting_height: u32,
    ) -> Result<SmartDesk, HardwareError> {
        let mut hal = IO::new();
        hal.configure_distance(<HcSr04 as Distance>::new());
        hal.configure_motor(
            <MotorObject<Motor<Resting>, Motor<Moving>> as MotorBuilder<
                Motor<Resting>,
                Motor<Moving>,
            >>::new(MotorRelayPins {
                gpio_relay_up_pin,
                gpio_relay_down_pin,
            })
            .expect("Failed to initialise relays"),
        );
        Ok(SmartDesk {
            hal,
            standing_height,
            resting_height,
        })
    }
    fn handle_moving_desk(
        hal: HAL,
        next_motion: TableMotion,
        target: f32,
    ) -> Result<HAL, SmartDeskError> {
        let start = Instant::now();
        let mut debounce_target = Instant::now();
        let mut debounce_abort = Instant::now();

        let mut new_hal = hal.start_move_desk(next_motion)?;

        loop {
            match new_hal
                .find_interaction(next_motion, target)
                .expect("Failed to find interaction")
            {
                Interaction::CloseObject => {
                    if Instant::now() - debounce_abort > Duration::from_secs(2) {
                        log::info!("cancelling motion");
                        break;
                    }
                }
                Interaction::NoInteraction => {
                    debounce_abort = Instant::now();
                    debounce_target = Instant::now();
                    if Instant::now() - start > Duration::from_secs(30) {
                        log::error!("timeout");
                        break;
                    }
                }
                Interaction::TargetMet | Interaction::TargetExceeded => {
                    if Instant::now() - debounce_target > Duration::from_millis(10) {
                        log::info!("Distance reached: {}cm", target);
                        break;
                    }
                }
            }
        }
        Ok(new_hal.stop_move_desk()?)
    }

    pub fn run(self) -> Result<(), HardwareError> {
        let start = Instant::now();
        let mut previous_motion = TableMotion::Rest;
        let mut hal = self.hal;

        loop {
            match hal.find_interaction(previous_motion, 0.0)? {
                Interaction::CloseObject => {
                    if Instant::now() - start > Duration::from_millis(100) {
                        let (next_motion, target) = match previous_motion {
                            TableMotion::Down | TableMotion::Rest => {
                                previous_motion = TableMotion::Up;
                                (previous_motion, self.standing_height as f32)
                            }
                            TableMotion::Up => {
                                previous_motion = TableMotion::Down;
                                (previous_motion, self.resting_height as f32)
                            }
                        };
                        hal = Self::handle_moving_desk(hal, next_motion, target)
                            .expect("Couldn't move desk");
                    }
                    continue;
                }
                Interaction::NoInteraction => {
                    log::trace!("No interaction yet");
                    thread::sleep(Duration::from_millis(500));
                    break;
                }
                Interaction::TargetMet | Interaction::TargetExceeded => {
                    log::trace!("Already at target");
                    break;
                }
            }
        }
        Ok(())
    }
}
