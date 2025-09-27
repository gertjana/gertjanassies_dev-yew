---
title: Using a e-ink display to make a wall calendar
author: Gertjan Assies
date: "2025-01-14"
category: code, make
tags: python, eink, development
published: false
image: "/static/images/eink-calendar.jpg"
summary: "Had an e-ink display laying around and some time"
---

Pimoroni's Inky Impression is a 7.3" 7 color e-ink display.

E-ink means it only uses energy to change the display, if you turn the power off, the display keep showing the latest content.

The downside is that refreshing takes a lot longer, in the case of this display around 40 seconds!!

This makes it usable for use-cases that will only need to update every now and then, so a screen by the door, that shows the next events from a calendar, the current weather and perhaps the mandatory inspirational quote it is then

On the back it has a 40 pin header that is compatiable with the Raspberry PI, so I stuck a Pi Zero 2W on the back and off we go.

It uses I2C to communicate with an eeprom, for display detection, and SPI to update the screen

Primoroni has created a python library to make it easier to use, I'm saying easier as under the hood it's using the Pillow (PIL) library to manipulate images, what you need to do is create an image, draw all your text, lines, icons etc on it and then tell it to display that image on the screen, which is quite a low level way to create an application.

For a few seconds I thought about creating a small UI library, that would allow me to use lists and tables etc, but that would take much more time and I'm just testing out this display, nothing more, so hardcoded positions with the occassional overlap it is.

## Starting with the library

```python
  from inky.auto import auto
  from PIL import Image, ImageDraw, ImageFont
  from font_source_sans_pro import SourceSansProSemibold

  display = auto(ask_user=True, verbose=True)
  image = Image.new("P", disp.resolution, disp.WHITE)

  canvas = ImageDraw.Draw(image)
  canvas.text((10, 10), "Hello World", disp.WHITE, ImageFont.truetype(SourceSansProSemibold, 48))

  display.set_image(image)
  display.show()
```

## Getting the data

### Google calendar

 Here I just took Google's python quickstart tutorial with the OAuth flow to get the events of a calendar and instead of printing it to the console, draw it on the canvas

```python
def getLatestEventsFromGoogleCalendar(maxEvents, cal_id):
  creds = Credentials.from_authorized_user_file("token.json", SCOPES)
  service = build("calendar", "v3", credentials=creds)
    now = datetime.datetime.utcnow().isoformat() + "Z"     events_result = (
      service.events()
      .list(
        calendarId=cal_id,
        timeMin=now,
        maxResults=maxEvents,
        singleEvents=True,
        orderBy="startTime",
      )
      .execute()
    )
  return events_result.get("items", [])
```
 ## The weather

 openweathermap.org has a nice API and subscription where you get a 1000 free calls a day

```python
def getWeather(api_key, lat, lon):
  url = 'https://api.openweathermap.org/data/3.0/onecall?units=metric&lat=' + lat + '&lon=' + lon +'&appid=' + api_key

  daily=requests.get(url).json()["daily"][0]
  weather = daily["weather"][0]
  return {
           "temp": daily["temp"]["day"],
           "feel": daily["feels_like"]["day"],
           "pressure": daily["pressure"],
           "humidity": daily["humidity"],
           "main": weather["main"],
           "desc": weather["description"],
           "icon": weather["icon"]
         }
```
## The quote

I use zenquotes as there are free
```python
def getQuote():
  quote = requests.get('https://zenquotes.io?api=random').json()[0]
  return f'"{quote["q"]}" - {quote["a"]}'
```
