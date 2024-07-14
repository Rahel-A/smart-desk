use super::hardware_error::HardwareError;
use super::TableMotion;

// motor
pub enum MotorObject<M: MoveAction, R: RestAction> {
    Moving(R),
    Resting(M),
}
pub trait MotorBuilder<M, R>
where
    M: MoveAction,
    R: RestAction,
{
    type InitParams;
    type Err;
    fn new(init: Self::InitParams) -> Result<MotorObject<M, R>, Self::Err>;
}

#[derive(Debug)]
pub enum SmartDeskError {
    WrongState,
}

pub trait MoveAction {
    type NextState;
    fn start_move_desk(self, direction: TableMotion) -> Result<Self::NextState, SmartDeskError>;
}
pub trait RestAction {
    type NextState;
    fn stop_move_desk(self) -> Result<Self::NextState, SmartDeskError>;
}

// distance
#[derive(Debug, PartialEq)]
pub enum Interaction {
    NoInteraction,
    CloseObject,
    TargetMet,
    TargetExceeded,
}

pub trait Interact {
    fn find_interaction(
        &mut self,
        motion: TableMotion,
        target: f32,
    ) -> Result<Interaction, HardwareError>;
}

pub trait Distance: Interact {
    type Object;
    fn new() -> Self::Object;
}

pub struct IO<M: MoveAction, R: RestAction, D: Distance> {
    motor: Option<MotorObject<M, R>>,
    distance: Option<D>,
}

impl<M: MoveAction, R: RestAction, D: Distance> IO<M, R, D> {
    pub fn new() -> IO<M, R, D> {
        IO {
            motor: None,
            distance: None,
        }
    }
    pub fn configure_distance(&mut self, distance_sensor: D) {
        self.distance = Some(distance_sensor)
    }
    pub fn configure_motor(&mut self, motor: MotorObject<M, R>) {
        self.motor = Some(motor)
    }
}

impl<M: MoveAction, R: RestAction, D: Distance> Interact for IO<M, R, D> {
    fn find_interaction(
        &mut self,
        motion: TableMotion,
        target: f32,
    ) -> Result<Interaction, HardwareError> {
        self.distance
            .as_mut()
            .ok_or(HardwareError::NotConfigured)?
            .find_interaction(motion, target)
    }
}
impl<M: MoveAction<NextState = R>, R: RestAction, D: Distance> MoveAction for IO<M, R, D> {
    type NextState = IO<M, R, D>;
    fn start_move_desk(self, direction: TableMotion) -> Result<Self::NextState, SmartDeskError> {
        match self.motor.unwrap() {
            MotorObject::Resting(obj) => {
                return Ok(IO {
                    motor: Some(MotorObject::Moving(obj.start_move_desk(direction)?)),
                    ..self
                })
            }
            MotorObject::Moving(_) => {
                log::trace!("Already moving");
                return Err(SmartDeskError::WrongState);
            }
        }
    }
}

impl<M: MoveAction, R: RestAction<NextState = M>, D: Distance> RestAction for IO<M, R, D> {
    type NextState = IO<M, R, D>;
    fn stop_move_desk(self) -> Result<Self::NextState, SmartDeskError> {
        match self.motor.unwrap() {
            MotorObject::Resting(_) => {
                log::trace!("Already Resting");
                return Err(SmartDeskError::WrongState);
            }
            MotorObject::Moving(obj) => {
                return Ok(IO {
                    motor: Some(MotorObject::Resting(obj.stop_move_desk()?)),
                    ..self
                });
            }
        }
    }
}
