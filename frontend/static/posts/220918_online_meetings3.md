---
title: All those online meetings - Part 3
author: Gertjan Assies
date: "2022-09-18"
category: make
tags: pico, kicad, fusion360, 3dprint, featured
image: "/static/images/online_meetings3_top.png"
summary: "In the previous two blogs, I explained the idea and execution of a small device that would allow me to mute/unmute audio/video and raise a hand in video calls with hangouts, zoom or teams.
now to wrap it up, and create an encasing for it"
published: true

---

<script lang="ts">
  import { Lightbox } from 'svelte-lightbox';
</script>

## Wrapping it up, in a case actually

In the previous two blogs, I explained the idea and execution of a small device that would allow me to mute/unmute audio/video and raise a hand in video calls with hangouts, zoom or teams.
Here are the links to both previous articles

Part 1: [All those online meetings - Part 1](/post/210619_online_meetings)
Part 2: [All those online meetings - Part 2](/post/220801_online_meetings2)

So it was working, and I was actually using it, but it was just a PCB (printed circuit board) laying on my desk. so I started thinking about creating a case for it. here's a picture of the PCB in its latest revision.
the Link in the references will take you to this board on Aisler's website.

<Lightbox><img alt="PCB" src="/static/images/online_meetings3_1.png" style="width:600px;" /></Lightbox>

I wanted the electronics to be visible so something transparent, which also would benefit from seeing the small OLED display I've added in the meantime.
So I decided to make the top and bottom 2 mm Acrylic.

so let's spin up Fusion360. this is by far the best 3D CAD design tool I've worked with, and it is free for personal use (the link in the references takes you to the “for personal use” page), you do have to go through some hoops every year when that personal license expires.

From the PCB design in [Kicad](https://www.kicad.org/), I exported the edges of the board, the drill holes in the corner as well as the edges of all the switches as a DXF file and imported that into Fusion360, now most designs in Fusion360 start with a 2D sketch.

<Lightbox><img alt="Sketch" src="/static/images/online_meetings3_2.png" style="width:600px;" /></Lightbox>

Which you then can extrude into the 3D shape required. and that allowed me to create the sides of the case. I extruded the sketch above two times, once with a height of 2mm for the part under the PCB and once with a height of 7mm for the part above the PCB.
making an opening for the USB connector was nothing more than creating a box a little bigger than the USB plug and then subtracting it from the shape of the case. the result is shown here, both the top and bottom are already in their correct position, and you can imagine the PCB fitting in between them.

<Lightbox><img alt="3D Model for the sides" src="/static/images/online_meetings3_3.png" style="width:600px;" /></Lightbox>

3D model for printing the sides of the case

I also uploaded a slightly different version of that exported dxf file to the [website](http://www.laserlokaal.nl) of a company that would laser-cut the top and bottom acrylic for me.

<Lightbox><img alt="lasercut dxf design" src="/static/images/online_meetings3_4.png" style="width:600px;" /></Lightbox>

The featured picture is a rendering done within Fusion360, all the 3D models for the electronic parts were found on grabcad.com except for the 3-way switch, but from the part shop I bought the part from, there was a possibility to render a CAD model for that part.
Fusion360 also can do animations. so here is one for an exploding view, which really shows you how it all fits together:

<Lightbox><img alt="Exploding view animation" src="/static/images/online_meetings3_5.gif" style="width:600px;" /></Lightbox>

So what did it all cost me?

| Item                 |  Qnty | Price | Remarks |
|----------------------|-----|------|------------------------|
| PCB                   | 1   |  €5.58   |Aisler (minimum order 3) |
| Acryl top+bottom      | 1   |  €12.30  |Laserlokaal |
| 3D Print top+bot sides| 1   |  €0.29   |(Filament cost) |
| RPI Pico RP2040       | 1   |  €2.20 |
| SSD1306               | 1   |  €2.20 |
| MX Keys               | 3   |  €3.03 |
| Switch                | 1   |  €0.65 |
| Key caps              | 3   |  €3.35 |
| M2x4 self tapping     | 4   |  €0.07 |
| M2x8 self tapping     | 4   |  €0.07 |

Total                        **€29.81**

All in all, I very much enjoyed this little project, for all the details, design files and source code go have a look at the repo:
[https://gitlab.com/gertjana/conference-buttons](https://gitlab.com/gertjana/conference-buttons)

How to program the board for the OLED display is something for another blog.

Hope you enjoyed it and maybe even inspired you.
Let me know if you have any questions or improvements.

## References

1. [Kicad](http://www.kicad.org) (Schematic and PCB Design)
2. [Aisler](https://aisler.net/p/HXAFRBPY) (online PCB Manufacturing)
3. [Laser Lokaal](http://www.laserlokaal.nl) (online Laser cutting/engraving)
4. [Fusion360](https://www.autodesk.com/products/fusion-360/personal) (Information about how to get a free personal license)
5. [Grabcad](https://grabcad.com/library) (Community sharing lots of 3D Models)
6. [Gitlab Repo](https://gitlab.com/gertjana/conference-buttons) (Contains Everything described here, Code, design files, etc)
