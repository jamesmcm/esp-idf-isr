# esp-idf-isr

A minimal implementation of a trait allowing to subscribe to interrupts on GPIO Pin of the ESP32.

## Example usage 

```rust
let (mut eventloop, _subscription) = init_eventloop().unwrap();

let peripherals = Peripherals::take().unwrap();
let mut interrupt_pin = peripherals.pins.gpio0
    .into_input().unwrap()
    .into_pull_up().unwrap();
interrupt_pin.configure_interrupt(InterruptType::NegEdge).unwrap();
let _subscription = unsafe {
    interrupt_pin.subscribe(move || {
        eventloop.post(&event::EventLoopMessage::new(1), None).unwrap();
    })?
};
```
