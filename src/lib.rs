mod callback;

use std::sync::atomic::{AtomicBool, Ordering};

use embedded_hal::digital::v2::InputPin;

use esp_idf_hal::gpio::{Input, Pin};
use esp_idf_sys::{esp, EspError, gpio_int_type_t, gpio_int_type_t_GPIO_INTR_ANYEDGE, gpio_int_type_t_GPIO_INTR_MAX, gpio_int_type_t_GPIO_INTR_DISABLE, gpio_int_type_t_GPIO_INTR_NEGEDGE, gpio_int_type_t_GPIO_INTR_LOW_LEVEL, gpio_int_type_t_GPIO_INTR_HIGH_LEVEL, gpio_int_type_t_GPIO_INTR_POSEDGE, gpio_set_intr_type, gpio_intr_disable, gpio_intr_enable};

/// This trait is used to represent a Pin on which you can subscribe using
/// a callback
pub trait InputPinNotify<'p>: InputPin + Pin {

    /// # Safety
    ///
    /// The callback passed to this method is executed in the context of an
    /// interrupt handler. So you should take care of what is done in it.
    unsafe fn subscribe(&'p mut self, callback: impl for<'a> FnMut() + 'static) -> Result<PinNotifySubscription<'p, Self>, EspError> where Self: Sized;
}

pub trait InterruptEnabled {
    fn configure_interrupt(&mut self, int_type: InterruptType) -> Result<(), EspError>;
}

macro_rules! impl_input_pin_notify {
    ($pin:ty) => {
        impl<'p> InputPinNotify<'p> for $pin {
            unsafe fn subscribe(&'p mut self, callback: impl for<'a> FnMut() + 'static) -> Result<PinNotifySubscription<'p, $pin>, EspError> {
                PinNotifySubscription::subscribe(self, callback)
            }
        }

        impl InterruptEnabled for $pin {
            fn configure_interrupt(&mut self, int_type: InterruptType) -> Result<(), EspError> {
                enable_isr_service()?;
                esp!(unsafe { gpio_set_intr_type(self.pin(), int_type.into()) })?;
                match int_type {
                    InterruptType::Disable => esp!(unsafe { gpio_intr_disable(self.pin()) }),
                    _ => esp!(unsafe { gpio_intr_enable(self.pin()) })
                }
            }
        }
    }
}

#[derive(Copy,Clone)]
pub enum InterruptType {
    AnyEdge,
    Max,
    Disable,
    NegEdge,
    PosEdge,
    LowLevel,
    HighLevel
}

impl From<InterruptType> for gpio_int_type_t {
    fn from(int_type: InterruptType) -> gpio_int_type_t {
        match int_type {
            InterruptType::AnyEdge => gpio_int_type_t_GPIO_INTR_ANYEDGE,
            InterruptType::Max => gpio_int_type_t_GPIO_INTR_MAX,
            InterruptType::Disable => gpio_int_type_t_GPIO_INTR_DISABLE,
            InterruptType::NegEdge => gpio_int_type_t_GPIO_INTR_NEGEDGE,
            InterruptType::PosEdge => gpio_int_type_t_GPIO_INTR_POSEDGE,
            InterruptType::LowLevel => gpio_int_type_t_GPIO_INTR_LOW_LEVEL,
            InterruptType::HighLevel => gpio_int_type_t_GPIO_INTR_HIGH_LEVEL
        }
    }
}

impl From<gpio_int_type_t> for InterruptType {
    #[allow(non_upper_case_globals)]
    fn from(int_type: gpio_int_type_t) -> Self {
        match int_type {
            gpio_int_type_t_GPIO_INTR_ANYEDGE => InterruptType::AnyEdge,
            gpio_int_type_t_GPIO_INTR_MAX => InterruptType::Max,
            gpio_int_type_t_GPIO_INTR_DISABLE => InterruptType::Disable,
            gpio_int_type_t_GPIO_INTR_NEGEDGE => InterruptType::NegEdge,
            gpio_int_type_t_GPIO_INTR_POSEDGE => InterruptType::PosEdge,
            gpio_int_type_t_GPIO_INTR_LOW_LEVEL => InterruptType::LowLevel,
            gpio_int_type_t_GPIO_INTR_HIGH_LEVEL => InterruptType::HighLevel,
            other => panic!("Unknown GPIO interrupt type: {}", other),
        }
    }
}

impl_input_pin_notify!(esp_idf_hal::gpio::Gpio0<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio1<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio2<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio3<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio4<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio5<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio6<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio7<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio8<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio9<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio10<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio11<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio12<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio13<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio14<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio15<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio16<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio17<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio18<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio19<Input>);
// impl_input_pin_notify!(esp_idf_hal::gpio::Gpio20<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio21<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio22<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio23<Input>);
// impl_input_pin_notify!(esp_idf_hal::gpio::Gpio24<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio25<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio26<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio27<Input>);
// impl_input_pin_notify!(esp_idf_hal::gpio::Gpio28<Input>);
// impl_input_pin_notify!(esp_idf_hal::gpio::Gpio29<Input>);
// impl_input_pin_notify!(esp_idf_hal::gpio::Gpio30<Input>);
// impl_input_pin_notify!(esp_idf_hal::gpio::Gpio31<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio32<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio33<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio34<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio35<Input>);

type ClosureBox = Box<Box<dyn for<'a> FnMut()>>;

/// The PinNotifySubscription represents the association between an InputPin and
/// a registered isr handler.
/// When the PinNotifySubscription is dropped, the isr handler is unregistered.
pub struct PinNotifySubscription<'p, P: Pin + InputPin>(&'p mut P, ClosureBox);

static ISR_SERVICE_ENABLED: AtomicBool = AtomicBool::new(false);

impl<'p, P: InputPin + Pin> PinNotifySubscription<'p, P> {
    fn subscribe(pin: &'p mut P, callback: impl for<'a> FnMut() + 'static) -> Result<Self, EspError> {
        enable_isr_service()?;

        let pin_number: i32 = pin.pin();

        let callback: Box<dyn for<'a> FnMut() + 'static> = Box::new(callback);
        let mut callback = Box::new(callback);

        let unsafe_callback = callback::UnsafeCallback::from(&mut callback);

        esp!(unsafe {
            esp_idf_sys::gpio_isr_handler_add(
                pin_number,
                Some(irq_handler),
                unsafe_callback.as_ptr(),
            )
        })?;

        Ok(Self(pin, callback))
    }

    /// Cancel this subscription, deregistering the isr handler and
    /// dropping the PinNotifySubscription
    pub fn unsubscribe(self) {}
}

impl<'p, P: InputPin + Pin> Drop for PinNotifySubscription<'p, P> {
    fn drop(self: &mut PinNotifySubscription<'p, P>) {
        esp!(unsafe { esp_idf_sys::gpio_isr_handler_remove(self.0.pin()) }).expect("Error unsubscribing");
    }
}

unsafe extern "C" fn irq_handler(unsafe_callback: *mut esp_idf_sys::c_types::c_void) {
    let mut unsafe_callback = callback::UnsafeCallback::from_ptr(unsafe_callback);
    unsafe_callback.call();
}

fn enable_isr_service() -> Result<(), EspError> {
    if ISR_SERVICE_ENABLED.compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed) == Ok(false) {
        if let Err(e) = esp!(unsafe { esp_idf_sys::gpio_install_isr_service(0) }) {
            ISR_SERVICE_ENABLED.store(false, Ordering::SeqCst);
            return Err(e);
        }
    }
    Ok(())
}
