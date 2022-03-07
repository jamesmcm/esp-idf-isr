mod callback;

use std::sync::atomic::AtomicBool;

use embedded_hal::digital::v2::InputPin;

use esp_idf_hal::gpio::{Input, Pin};
use esp_idf_sys::{esp, EspError};

/// This trait is used to represent a Pin on which you can subscribe using
/// a callback
pub trait InputPinNotify: InputPin + Pin {

    /// # Safety
    ///
    /// The callback passed to this method is executed in the context of an
    /// interrupt handler. So you should take care of what is done in it.
    unsafe fn subscribe(self, callback: impl for<'a> FnMut() + 'static) -> Result<PinNotifySubscription<Self>, EspError> where Self: Sized;
}

macro_rules! impl_input_pin_notify {
    ($pin:ty) => {
        impl InputPinNotify for $pin {
            unsafe fn subscribe(self, callback: impl for<'a> FnMut() + 'static) -> Result<PinNotifySubscription<$pin>, EspError> {
                PinNotifySubscription::subscribe(self, callback)
            }
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
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio21<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio22<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio23<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio25<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio27<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio32<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio33<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio34<Input>);
impl_input_pin_notify!(esp_idf_hal::gpio::Gpio35<Input>);

type ClosureBox = Box<Box<dyn for<'a> FnMut()>>;

/// The PinNotifySubscription represents the association between an InputPin and
/// a registered isr handler.
/// When the PinNotifySubscription is dropped, the isr handler is unregistered.
pub struct PinNotifySubscription<P: Pin + InputPin>(P, ClosureBox);

static ISR_SERVICE_ENABLED: AtomicBool = AtomicBool::new(false);

impl<P: InputPin + Pin> PinNotifySubscription<P> {
    fn subscribe(pin: P, callback: impl for<'a> FnMut() + 'static) -> Result<Self, EspError> {
        if !ISR_SERVICE_ENABLED.load(std::sync::atomic::Ordering::SeqCst) {
            enable_isr_service()?;
            ISR_SERVICE_ENABLED.store(true, std::sync::atomic::Ordering::SeqCst);
        }
        let pin_number: i32 = pin.pin();

        esp!(unsafe { esp_idf_sys::rtc_gpio_deinit(pin_number) })?;
        esp!(unsafe {
            esp_idf_sys::gpio_set_intr_type(pin_number, esp_idf_sys::gpio_int_type_t_GPIO_INTR_ANYEDGE)
        })?;

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

impl<P: InputPin + Pin> Drop for PinNotifySubscription<P> {
    fn drop(self: &mut PinNotifySubscription<P>) {
        esp!(unsafe { esp_idf_sys::gpio_isr_handler_remove(self.0.pin()) }).expect("Error unsubscribing");
    }
}

unsafe extern "C" fn irq_handler(unsafe_callback: *mut esp_idf_sys::c_types::c_void) {
    let mut unsafe_callback = callback::UnsafeCallback::from_ptr(unsafe_callback);
    unsafe_callback.call();
}

fn enable_isr_service() -> Result<(), EspError> {
    esp!(unsafe { esp_idf_sys::gpio_install_isr_service(0) })
}
