---
title: Rust on ESP32 part 2.5 - Switching software to Rust no_std and Embassy
author: Gertjan Assies
date: "2025-10-03"
category: code, make
tags: rust, embedded, esp32, featured, embassy
published: true
image: "/static/images/rust_embassy.png"
summary: "Rewriting the source code to better fit the ESP32 using no_std Rust and Embassy framework"
---

> This is a series of articles about using Rust to program an ESP32 Microcontroller by building a minimal EV Charger.<br/>
>  * Part 1: [A Proof of Concept](/post/240101_rust_on_esp32)
>  * Part 2: [A minimal EV Charger hardware setup](/post/240125_rust_on_esp32_2_hardware)
>  * Part 2.5: [A rewrite using Rust no_std and the Embassy framework](post/240226_rust_on_esp32_2_5_embassy_no_std) (this article)
>  * Part 3: Network and Charger to backoffice communication (Wifi/ MQTT / OCPP)
>  * Part 4: Optional: Charger to Car communication (Mode2)


## The old

It took me a while to get back to this, work and personal life took priority, but I've picked up this project again and rewrote it completely

As you can see in the previous article, I was using standard Rust and was very low level spawning threads and sharing data through Mutexes

Which meant I had to do most of the plumbing myself, which could lead to bugs and probably race-conditions.

```rust
let org_charger: Arc<Mutex<Charger>> = Arc::new(Mutex::new(Charger{id: "1", state: "Available"}));

let charger = org_charger.clone();
thread:spawn(move || {
    loop {
        // do something with the charger
    }
});
```
As the `Charger` is wrapped in an `Arc` (Atomically Reference Counted) each clone will produce a new `Arc` Instance which points to the same allocation while increasing a reference count. The mutex is needed to keep the underlying Charger accessible to only one Thread at a time.

## The new

But this is all water under the bridge as I changed the implementation to use the Embassy Framework, which is similar to other async frameworks but specifically for small microcontrollers.

I also switched to `no_std` which is Rust without the standard library. This practically means no heap allocations, so we use `&str` instead of `String`

The above code will now look like this:

```rust

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let charger = mk_static!(Charger, Charger::new()); //the make_static! macro allows creating static variables initialized at runtime
    spawner.spawn(some_async_task(charger)).ok();
}

#[embassy_executor::task]
async fn some_async_task(charger) {
    loop {
        // do something with the charger
        Timer::after(Duration::from_millis(100)).await; // in case of non-blocking code wait a bit
    }
}
```

This is more in line with how most async frameworks like Tokio work.

Communication between Tasks is done with Channels where you are able to safely send and receive messages from other tasks which come in 2 flavours:
 - many sender to 1 receiver
 - 1 publisher to many subscribers

The Charger is essentially a State Machine where certain events (cable inserted, card swiped) cause state transitions that produce other events (cable lock, current turns on, etc.)

For instance, if the Charger's state is `Available` and someone inserts a cable, the task watching that event will send a message to the state machine, which will change its state to `Preparing` and broadcast that state change to a PubSubChannel. A task responsible for controlling the LED color will then change the color from green to white.
> Note that that task is in waiting state until a message comes in, taking no cpu time what so ever or need delays to 'free up' the thread.


### Channels workings

##### Normal channel
A normal channel can have multiple tasks putting messages in there, in our case changes in the hardware that will cause state changes.

```rust
/// Message queue for charger input events
pub static STATE_IN_CHANNEL: Channel<CriticalSectionRawMutex, InputEvent, 10> = Channel::new();

// task sending a message
#[embassy_executor::task]
async fn cable_detect_task(charger) {
    ...
    STATE_IN_CHANNEL.send(InputEvent.CableInserted).await;
    ...
}

// task sending a message
#[embassy_executor::task]
async fn receiving_task(charger) {
    ...
    let event = STATE_IN_CHANNEL.receive().await; // blocks until message arrives
    ...
}
```

##### Publish/Subscribe:
In our case state changes that needs to be propagated to the hardware
```rust
/// PubSub channel for charger state changes
pub static STATE_PUBSUB: PubSubChannel<
    CriticalSectionRawMutex,
    (ChargerState, heapless::Vec<OutputEvent, 2>),
    10, // capacity
    6, // max subscribers
    1, // max publishers
> = PubSubChannel::new();

// task that publishes
#[embassy_executor::task]
async fn publish_task() {
    let publisher = STATE_PUBSUB.publisher().unwrap();
    loop {
        ...
        publisher.publish_immediate(some_message);
    }
}

// task subscribed to the pubsub channel
#[embassy_executor::task]
async fn subscribe_task() {
    ...
    let mut subscriber = charger::STATE_PUBSUB.subscriber().unwrap();
    loop {
        if let embassy_sync::pubsub::WaitResult::Message((new_state, output_event)) =
            subscriber.next_message().await // this blocks until a message has been received
            // do something with the message
        {
    }
    ...
}
```

so we have:
 - Tasks reacting to things happening on the hardware site (Insert Cable, Swipe Card, etc.)
 - A charger state machine task that changes state based on these events (Available, Preparing, Charging, etc.)
 - Tasks that listen to these state changes and interact with the hardware (Led that set the right color, Solenoid locking cable, Relay switching Power to the Cable, etc.)
 - Tasks that communicate through MQTT to an OCPP Backend, provide a Wifi network stack, queries an NTP server for the current time, but thats for the next article.

## The State Machine

Here's how the state machine looks:
[![State Machine](/static/images/charger_state_machine.png)](/static/images/charger_state_machine.png)

And if you're brave, here's the complete architecture, (every green or blue rectangle is an embassy task, all communication between tasks go through channels)
<a href="/static/images/charger_architecture.png" target="_blank">
![Architecture](/static/images/charger_architecture.png#small)
</a>

## Hardware

I changed the ESP32 from an S3 (Xtensa CPU Architecture) to a C6 (RISC-V Architecture), primarily to avoid having to install the entire Xtensa SDK on my laptop. This also dramatically improved compilation times from a couple of minutes to around 35 seconds.

I added an addressable Multi color LED ([WS1218](https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf)) and a RFID Reader ([RC522](https://www.handsontec.com/dataspecs/RC522.pdf))

### The updated schematic:

[![Schematic](/static/images/charger_schematic.png)](/static/images/charger_schematic.png)

### A nice 3D Rendering made with Fusion360

[![3D render](/static/images/3d_render.png)](/static/images/3d_render.png)

### Front with Acrylate cover
The 3D printed part covers the pins that might contain high voltages.
[![Front PCB](/static/images/charger_front.png)](/static/images/charger_front.png)

### Back showing ESP32-C6, connectors and Relay
[![Back PCB](/static/images/charger_back.png)](/static/images/charger_back.png)

You can find all the schematic and PCB files in the kicad subdirectory of the [project](https://github.com/gertjana/charger-esp32c6-embassy/tree/main/kicad)


## The Result

 - We now have an application that is very responsive to what's happening
 - Embassy handles the concurrency, which will reduce bugs and possible race conditions
 - It will handle the scarce resources of embedded microcontroller's a lot better

## Conclusion

It's possible to write safe async code in rust on ESP32 microcontrollers.
Working within the `no_std` constraints was more manageable than expected, especially with the help of crates like 'alloc' and 'heapless' which help with alternative ways of allocation where needed.

The Embassy framework helps me to use the same programming concepts as in normal applications using for instance the Tokio async runtime.

For future articles, I will discuss network communication, setting up Wifi, getting the time from an NTP Server and talking to a backoffice using MQTT and the OCPP Protocol

If you're interested in getting started with Embassy and Rust on microcontrollers, I highly recommend the [Embassy book](https://embassy.dev/book/dev/index.html) as an excellent starting point.

## References
* Code/Design files: https://github.com/gertjana/charger-esp32c6-embassy/
* ESP32-C6 microcontroller: https://wiki.seeedstudio.com/xiao_esp32c6_getting_started/
* Embassy Framework: https://embassy.dev/
