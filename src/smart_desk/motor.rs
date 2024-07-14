use super::{
    hal::{MotorBuilder, MotorObject, MoveAction, RestAction, SmartDeskError},
    TableMotion,
};
use log::trace;
use rppal::gpio::{Gpio, OutputPin};
use std::{error::Error, marker::PhantomData};

pub struct Moving;
pub struct Resting;

pub struct Motor<T> {
    gpio_relay_up: OutputPin,
    gpio_relay_down: OutputPin,
    _state: PhantomData<T>,
}

pub struct MotorRelayPins {
    pub gpio_relay_up_pin: u8,
    pub gpio_relay_down_pin: u8,
}
impl MotorBuilder<Motor<Resting>, Motor<Moving>> for MotorObject<Motor<Resting>, Motor<Moving>> {
    type InitParams = MotorRelayPins;
    type Err = Box<dyn Error>;
    fn new(
        pins: Self::InitParams,
    ) -> Result<MotorObject<Motor<Resting>, Motor<Moving>>, Self::Err> {
        let gpio_relay_up = Gpio::new()?.get(pins.gpio_relay_up_pin)?.into_output_low();
        let gpio_relay_down = Gpio::new()?
            .get(pins.gpio_relay_down_pin)?
            .into_output_low();

        Ok(MotorObject::Resting(Motor {
            gpio_relay_up,
            gpio_relay_down,
            _state: PhantomData,
        }))
    }
}

impl From<Motor<Resting>> for Motor<Moving> {
    fn from(item: Motor<Resting>) -> Self {
        Motor {
            gpio_relay_up: item.gpio_relay_up,
            gpio_relay_down: item.gpio_relay_down,
            _state: PhantomData,
        }
    }
}

impl From<Motor<Moving>> for Motor<Resting> {
    fn from(item: Motor<Moving>) -> Self {
        Motor {
            gpio_relay_up: item.gpio_relay_up,
            gpio_relay_down: item.gpio_relay_down,
            _state: PhantomData,
        }
    }
}

impl MoveAction for Motor<Resting> {
    type NextState = Motor<Moving>;
    fn start_move_desk(
        mut self,
        direction: TableMotion,
    ) -> Result<Self::NextState, SmartDeskError> {
        match direction {
            TableMotion::Up => {
                trace!("Up");
                self.gpio_relay_up.set_high();
                self.gpio_relay_down.set_low();
                Ok(Motor { ..self.into() })
            }
            TableMotion::Down => {
                trace!("Down");
                self.gpio_relay_down.set_high();
                self.gpio_relay_up.set_low();
                Ok(Motor { ..self.into() })
            }
            _ => {
                trace!("Already resting");
                Err(SmartDeskError::WrongState)
            }
        }
    }
}

impl RestAction for Motor<Moving> {
    type NextState = Motor<Resting>;
    fn stop_move_desk(mut self) -> Result<Self::NextState, SmartDeskError> {
        trace!("Rest");
        self.gpio_relay_down.set_low();
        self.gpio_relay_up.set_low();
        Ok(Motor { ..self.into() })
    }
}
