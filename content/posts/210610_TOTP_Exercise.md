---
title: TOTP Exercise
author: Gertjan Assies
date: "2021-06-10"
category: code
tags: totp, tips, python
image: "/content/images/totp_exercise.jpg"
summary: As an exercise, I wanted to see if I could get TOTP codes used for multi factor authentication visible without having to resort to the Google Authenticator app.
published: true

---

TOTP stands for [Time-based One Time Password](https://en.wikipedia.org/wiki/Time-based_One-Time_Password) and is an algorithm that most MFA ([Multi-Factor Authentication](https://en.wikipedia.org/wiki/Multi-factor_authentication)) devices use

for obvious security reasons, I have set up MFA on all accounts that support it. but this can be a bit cumbersome in some situations.

Especially when you are working with Serverless applications on AWS. I work a lot with Cloudformation and SAM but that does mean that during the day I'll be typing in MFA codes a lot.

As an exercise, I wanted to see if I could get those codes visible without having to resort to the Google Authenticator app.

What if I could have that code show up in the touch bar on my mac.
I know, I know MFA needs to be another device! so a disclaimer:

> What is described below will lower the security of your system and your accounts: do so at your own risk!

but if I would do this, at least I would need to do the following:

* use sudo to enable it and disable it automatically after a certain amount of time.
* only shows it in the touch bar after pressing a secret key combination.
* store any secrets in the keychain.

I'm already using a tool called [BetterTouchTool](https://folivora.ai/) that allows customisation of the touch bar on my mac.

so first of all how do we get that code?

Whenever you add an application to for instance Google Authenticator, you have to scan a QR Code. this QR Code contains a secret if you would scan that with a normal QR code scanner you will get this URL with a secret param: (the account and secret are replaced by placeholders here)

```sh
otpauth://totp/Amazon%20Web%20Services:\[account\]?secret=\[secret\]&issuer=Amazon%20Web%20Services
```

Now here you can see that it uses TOTP (Timebase One Time Password) which is an extension to HOTP (HMAC-Based One Time Password)

to summarize really quickly:

HOTP uses a hash algorithm to create a digest from the secret and a counter) TOTP introduces a time-based component for the counter (the counter is x second (default is 30) steps counted from the epoch)

Now Python has an [OTP library](https://github.com/pyauth/pyotp) that can create the MFA code from that secret.

```python
#!/usr/bin/env python3import pyotp
import syssecret = sys.argv\[1\]
totp = pyotp.TOTP(secret)
print(totp.now())
```

This will when you give it a secret return the 6 digit code

To not have the secret in my scripts I've added it to the keychain with

```sh
security add-generic-password -a \[account\] -s totp-exercise -w \[secret\]
```

and retrieve it again in a bash script

```bash
#!/bin/bash
me=\`whoami\`
secret=\`security find-generic-password -a $me -s totp-exercise -w\`
echo \`python3 \[path\]/get\_code.py $secret\`
```

so now I have something I can call from BetterTouchTool:

In BetterTouchTool you can create a button (widget) and then run an AppleScript when started and another one when you press the button, you can also define a key combination to only show it when pressed.

so when starting it It will run this AppleScript: it will execute the script and set the text property with the result to have it shown as the button text

```applescript
set code to do shell script "/Users/\[account\]/btt\_scripts/code\_aws.sh"
return "{\\"text\\":\\"" & code & "\\"}"
```

when pressed it gets the latest code again but now put the code on the clipboard

```applescript
set code to do shell script "/Users/\[account\]/btt\_scripts/code\_aws.sh"
set the clipboard to code as text
```

To conclude with the pytotp library it is trivial to build your own google authenticator application.

Now that I've "proofed the concept" I will remove it again!

And focus on getting this to run on an embedded device, for instance on an [M5 Stack](https://m5stack.com/) which has an ESP32 controller and a screen. although this will probably mean I have to port the python library to C, as I haven't found any libraries that work with the AWS secret, just some abandoned projects that tried to do the same.
But that's another story
