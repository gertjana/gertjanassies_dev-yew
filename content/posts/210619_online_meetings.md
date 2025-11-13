---
title: All those online meetings - part 1
author: Gertjan Assies
date: "2021-06-19"
category: make
tags: python, micropython, rp2040, keyboard
image: "/content/images/online_meetings_top.png"
summary: "While on a not particularly interesting online meeting, my eye fell on the [Raspberry Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/) that I ordered and had just arrived that morning.
After more than a year working from home, I got into the habit to mute my audio when I'm not speaking and in large meetings or perceived bandwidth issues, I disable my video. enabling both again when I need to speak.
Now, what if I could add a couple of buttons to the Pico to make those tasks a little easier."
published: true

---

_Or how to find an excuse to play with a Raspberry Pico._

While on a not particularly interesting online meeting, my eye fell on the [Raspberry Pico](https://www.raspberrypi.org/products/raspberry-pi-pico/) that I ordered and had just arrived that morning.
After more than a year working from home, I got into the habit to mute my audio when I'm not speaking and in large meetings or perceived bandwidth issues, I disable my video. enabling both again when I need to speak.

Now, what if I could add a couple of buttons to the Pico to make those tasks a little easier.

Add another button to switch between hangouts/zoom/teams and I should be good to go. thus making the pico into a 3 button keyboard.

The Pico or more specifically the RP2040 is a $4 microcontroller board that features a dual-core Arm Cortex-M0+ processor with 264KB internal RAM and support for up to 16MB of off-chip Flash. I2C, SPI, and programmable I/O. making it more than suitable for this small project.

<Image path="/content/images/online_meetings_1.png" alt="RP2040 pinlayout" thumbnail_width="600" />

A quick search on the internet revealed an [AdaFruit library](https://circuitpython.readthedocs.io/projects/hid/en/latest/) that would turn the Pico into an HID (Human Interface Device) which would make it a keyboard when plugged in the USB port. that library also came with a [tutorial](https://learn.adafruit.com/diy-pico-mechanical-keyboard-with-fritzing-circuitpython/overview) for a 21-key keyboard, which I used extensively in this little project. (no point in inventing the wheel again)

The tutorial also contains custom parts for the RP2040 and the Cherry MX buttons to be used in [Fritzing](https://fritzing.org/), a simple tool to create schematics and PCB's.

## Hardware

<Image path="/content/images/online_meetings_2.png" alt="Schematic" thumbnail_width="600" />

So simply adding 3 switches to 3 GPIO ports, in the software, we can define them as input with the internal pull-up resistor which will make them work correctly.

Switching to the PCB view in Fritzing, placing the components and routing the switches to the pico and creating a copper fill for the ground connections resulted in the following PCB:

<Image path="/content/images/online_meetings_3.png" alt="PCB" thumbnail_width="600" />

Fritzing also supports exporting to Gerber files. which u can use to create your PCB's or order them online. this process could not be easier, upload the Gerber files, visually check them online and press the order button.

I used [OSHPark](https://oshpark.com/) for the first batch (you need to order a minimum of 3 PCB's), they deliver a great product, but it takes around a month to manufacture and ship from the US to the Netherlands, costs were around $25 which is not bad.
For the second batch, I'm trying out [Aisler](https://aisler.net/), which originates in Germany, and is cheaper (12 euro for 3 boards) and should be here in less than 2 weeks.

To program the Pico with Micropython, it always consists of resetting it with the bootsel button pressed which will make it show up as a removable drive on your system then copy/drag over the necessary files and that it, details are in the adafruit tutorial I've linked above and in the references.

## Software

All the software, design files can be found here:
[https://gitlab.com/gertjana/conference-buttons](https://gitlab.com/gertjana/conference-buttons)

I'm using [Mu Editor](https://codewith.mu/), it's not the best editor, but it supports the Pico board and the circuit python bootloader to directly program on the Pico.

One thing in the code I want to highlight to show the versatility of the Pico:
to be able to use the GPIO's as seen in the schematic, they need to be set up as inputs with pull-up resistors

This allows you to program almost every pin, as digital in or out, analog, PWM, I2C, SPI, UART, making it very versatile to many use cases.

The rest of the code just loops forever, scanning the keys 10x a second and when pressed lookup the correct keyboard shortcut to send over the USB connection.

## Conclusion

This approach demonstrated here (with the help of the good people at Adafruit) lends itself to creating custom keyboards for all kinds of situations, up to complete cockpits for flight simulators or other game-specific keyboards.

The Raspberry Pi Foundation clearly has a winner here, a very cheap versatile microcontroller with the power of the dual-core Arm M0 and good support from a lot of manufacturers like Adafruit and others making it an in my eyes one of the best options out there for small DIY projects and education purposes.
I can't wait to have my niece and nephew solder one together for their online school lessons, as I have still 2 of the 3 boards left.

Continued in Part 2: [All those online meetings - part 2](/blog/220801_online_meetings2)

## References

* [Raspberry Pico RP2040](https://www.raspberrypi.org/products/raspberry-pi-pico/)
* [Adafruit CircuitPython and HID Library bundle](https://circuitpython.org/libraries)
* [Adafuit custom keyboard tutorial](https://learn.adafruit.com/diy-pico-mechanical-keyboard-with-fritzing-circuitpython/overview)
* [Fritzing opensource schematic and PCB designer](https://fritzing.org/)
* [Mu Python Editor](https://codewith.mu/)
* [Source and Fritzing schematic and PCB Files](https://gitlab.com/gertjana/conference-buttons)
* [PCB's in Aisler](https://aisler.net/p/KHQPFARM)
