---
title: Rust on ESP32 part 3 - MQTT and OCPP
author: Gertjan Assies
date: "2024-02-26"
category: code, make
tags: rust, embedded, esp32, featured, ocpp, mqtt
published: false
image: "/static/images/charger_mqtt.png"
summary: "Communicating with a backend using MQTT and OCPP"
---

<script lang="ts">
    import { Lightbox } from 'svelte-lightbox';
</script>

> This is a series of articles about using Rust to program an ESP32 Microcontroller by building a minimal EV Charger.<br/>
>  * Part 1: [A Proof of Concept](/blog/240101_rust_on_esp32)
>  * Part 2: [A minimal EV Charger hardware setup](/blog/240125_rust_on_esp32_2_hardware)
>  * Part 3: Network and Charger to backoffice communication (Wifi/ MQTT / OCPP) (this article)
>  * Part 4: Optional: Charger to Car communication (Mode2)


## references
* Code: https://github.com/gertjana/charger_rust_esp32_c3/tree/cad15fb3a088cdd82d7dbfd5f9d16512b37e4d6f
* esp-idf-template: https://github.com/esp-rs/esp-idf-template
* M5 Stamp ESP32-C3U: https://docs.m5stack.com/en/core/stamp_c3u
* The embedded rust book:  https://docs.rust-embedded.org/book/
* Espresiff ESP-32: https://www.espressif.com/en/products/socs/esp32
