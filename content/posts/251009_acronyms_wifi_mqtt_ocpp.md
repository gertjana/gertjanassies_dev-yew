---
title: Rust on ESP32 part 3 - Connecting to a backoffice (Wifi/MQTT/OCPP)
author: Gertjan Assies
date: "2025-10-09"
category: code, make
tags: rust, embedded, esp32, featured, mqtt, ocpp
published: false
image: "/content/images/charger_connect_cloud.png"
summary: "How to connect to a backoffice"
---

> This is a series of articles about using Rust to program an ESP32 Microcontroller by building a minimal EV Charger.<br/>
>  * Part 1: [A Proof of Concept](/post/240101_rust_on_esp32)
>  * Part 2: [A minimal EV Charger hardware setup](/post/240125_rust_on_esp32_2_hardware)
>  * Part 2.5: [A rewrite using Rust no_std and the Embassy framework](/post/240226_rust_on_esp32_2_5_embassy_no_std)
>  * Part 3: [Network and Charger to backoffice communication (Wifi/ MQTT / OCPP)](/post/251009_acronyms_wifi_mqtt_ocpp)  (this article)
>  * Part 4: Optional: Charger to Car communication (Mode2)

I've worked in e-mobility for over 11 years now, and from the moment I stepped into this market, I always felt the way chargepoints connect to a backoffice was less then optimal.

Chargepoints connect to a backoffice through a websocket connetion, this means every chargepoint has to have an active connection with a server. although this allows for bi-directional communication, it is ultimately not scalable.

I always thought about chargepoints as IoT (Internet of Things), multiple devices in the field, being able to communicate with them, change settings, update firmware etc.

And IoT already has a communication solution for this called MQTT (Message Queueing Telemetry Transport)

## MQTT

From Wikipedia:

> MQTT is  a lightweight, publish–subscribe, machine-to-machine network protocol for message queue/message queuing service. It is designed for connections with remote locations that have devices with resource constraints or limited network bandwidth, such as in the Internet of things (IoT). It must run over a transport protocol that provides ordered, lossless, bi-directional connections—typically, TCP/IP.[1] It is an open OASIS standard and an ISO recommendation (ISO/IEC 20922).

So that's why I decided for this project to move away from continous websocket connection to the more asynchronous pub/sub model that MQTT is.
