---
title: All those online meetings - part 2
author: Gertjan Assies
date: "2022-08-01"
category: make
tags: pico, kicad, python
image: "/static/images/online_meetings2_top.png"
summary: "Some improvements to the original design"
published: true

---

<script lang="ts">
    import { Lightbox } from 'svelte-lightbox';
</script>
A while ago I blogged about a small project with a Raspberry Pi Pico and a couple of buttons to make a tool that could help me in all those online meetings when we all had to work from home during Covid.

I'm just explaining the changes here, so If you want to get the full picture, please read the first part of the article:  
[All those online meetings - part 1](/blog/210619_online_meetings)

Now it works pretty well but there were some improvements to be made  

* I needed one of the buttons to do a "raise hand"
* Because of that I needed another way to switch between Hangouts/Teams/Zoom

A 3-way slider switch should do the trick, now my schematic looks like this (power comes from the USB connector):

<Lightbox><img alt="Schematic" src="/static/images/online_meetings2_1.png" /></Lightbox>

All I need is a change in the code to check GPIO13, 14 and 15 and whichever one is 0/false/low I had to set the keymap for the right conference tool.  
Because I configured the GPIO ports to use a pull-up resistor an open switch will be 1 (resistor pulls the input to the + Voltage) and closing the switch will pull it to 0 (ground) with the resistor protecting it from shortening out.

```python
switch = [board.GP13, board.GP14, board.GP15]
for s in range(3):
    switch[s] = DigitalInOut(switch[s])
    switch[s].direction = Direction.Input
    switch[s].pull = Pull.Up

keymap = {}
keymap_teams    = { (0): (KeyCode.CONTROL, KeyCode.SHIFT, KeyCode.M), 
                    (1): (KeyCode.CONTROL, KeyCode.SHIFT, KeyCode.O),
                    (2): (KeyCode.CONTROL, KeyCode.SHIFT, KeyCode.K)}
keymap_zoom     = { (0): (KeyCode.CONTROL, KeyCode.SHIFT, KeyCode.A), 
                    (1): (KeyCode.GUI, KeyCode.SHIFT, KeyCode.V),
                    (2): (KeyCode.GUI, KeyCode.Y)}
keymap_hangouts = { (0): (KeyCode.ALT, KeyCode.D), 
                    (1): (KeyCode.GUI, KeyCode.E),
                    (2): (KeyCode.GUI, KeyCode.CONTROL, KeyCode.H)}

def get_keymap():
    if not switch[0].value:
        return keymap_teams
    if not switch[1].value:
        return keymap_zoom
    if not switch[2].value:
        return keymap_hangouts

keymaps = get_keymap()
```

Now about the hardware, the first version was done with Fritzing which is a nice enough tool. but it has some peculiarities, that make it hard to get right.  
So for this one, I used [KiCad 6](https://www.kicad.org/) and I must say it has evolved a lot since I first started using it ages ago.

One thing with Electronic design software is always having the right part libraries and KiCad delivers here. I'm using Cherry MX keys that I have lying around. and those are fully supported. For the Raspberry Pi Pico board, a quick google revealed a [GitHub repo](https://github.com/ncarandini/KiCad-RP-Pico)f with the part and PCB footprint

The switch part took a bit longer, I had already ordered and received the switch, but it wasn't in the library, fortunately, its little brother a 2-way switch was. as you can see in the screenshot

<Lightbox><img alt="PCB_key_2way" src="/static/images/online_meetings2_3.png" style="width:400px" /></Lightbox>

Now the switch I had looked exactly the same but it had one more pin and some more spacing.  
So in KiCad's footprint editor, I copied the one above. saw that the pad spacing was 2 mm. so I could move on off the outer pins to the left and duplicate one of the other pins. update the pin numbers in the properties to match up with the 3-way switch schematic part. and for completeness updated the silkscreen mask and part number.

<Lightbox><img alt="PCB_key_3way" src="/static/images/online_meetings2_4.png" style="width:400px" /></Lightbox>

Now I have all the parts that I need, in the schematic, I assigned all the footprints to the relevant parts and after satisfying the Electrical Rules Checker I clicked Update PCB from Schematic.

Now in the PCB editor it's just placing the components and routing everything but the GND on the top layer. and then creating a ground plane for the GND net on the bottom layer.  
I had to make one via to get the GND from the raspberry pi board which will be soldered on top to the bottom layer and part of the ground plane.  
Running the Design Rule Checker revealed some warnings but overall it was happy with my design.

Here's the end result:

Last time I ordered the PCB boards from [Aisler](https://aisler.net/). and I found out that they have a plugin for KiCad, which makes it super easy to upload the PCB to Aisler (one click) and from there. a quick check, going over all the settings, the defaults were good enough. and pressing the produce button. and then waiting a week or so. I had to pay around 15 Euro. which is a really good value for the quality they deliver.

I'm very impressed with KiCad, of course having used other electronic design tools in the past (Ultiboard, Eagle. Fritzing) made it a bit easier. but I felt like KiCad had the most gradual learning curve of them all.  
I managed to create the schematic, PCB and extra footprint in about 3 hours.

All the code and designs are here: [https://gitlab.com/gertjana/conference-buttons](https://gitlab.com/gertjana/conference-buttons)

Thanks for reading. and hopefully, I inspired you to start doing your own little projects.

next steps: Create a 3D printed case for it…. _is firing up Fusion360_

Continueing in Part 3:  
[All those online meetings - Part 3](/blog/220918_online_meetings3)
