use std::{thread, time::Duration};

use hc_sr04::{HcSr04, Unit};

use super::{hal::{Distance, Interact, Interaction}, hardware_error::HardwareError, TableMotion};

impl Distance for HcSr04 {
    type Object = HcSr04;
    fn new() -> Self::Object {
        const GPIO_TRIG: u8 = 24; // pin#18
        const GPIO_ECHO: u8 = 25; // pin#22
        HcSr04::new(GPIO_TRIG, GPIO_ECHO, None).expect("ultrasonic GPIO setup failed")
    }
}
fn within_margin(distance: f32, programmed_height: f32) -> bool {
    return (distance - programmed_height).abs() < 5.0 /* cm */;
}

impl Interact for HcSr04 {
    fn find_interaction(
        &mut self,
        motion: TableMotion,
        target: f32,
    ) -> Result<Interaction, HardwareError> {
        /* Important not to spam the HC-S4 */
        thread::sleep(Duration::from_millis(5));
        if let Ok(maybe_distance) = self.measure_distance(Unit::Centimeters) {
            if let Some(distance) = maybe_distance {
                let interaction = if distance < 20.0 {
                    Interaction::CloseObject
                } else if within_margin(distance, target) {
                    Interaction::TargetMet
                } else if (TableMotion::Up == motion && distance > target)
                    || (TableMotion::Down == motion && distance < target)
                {
                    Interaction::TargetExceeded
                } else {
                    Interaction::NoInteraction
                };
                log::debug!("Determined interaction={interaction:?} from dist={distance}cm");
                return Ok(interaction);
            }
            return Err(HardwareError::DistanceOutOfRange);
        }
        return Err(HardwareError::NoDistance);
    }
}
