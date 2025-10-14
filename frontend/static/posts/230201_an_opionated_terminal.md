---
title: An opinionated terminal experience
author: Gertjan Assies
date: "2023-02-01"
tags: terminal, fish, iterm2, brew
category: tooling
image: "/static/images/an_opinionated_terminal_top.jpg"
summary: "As a Site Reliability / Software Engineer I'm a bit opinionated when it comes to the tool I use most of the time: The command line."
published: true

---

As I got a new work laptop (Macbook Pro M1) recently, I've recorded the steps I did to get it to behave as I want, and hopefully, be a little bit more productive. (I'm very aware of the delicate balance between spending time on being more productive and actually being productive)

It all starts with [homebrew](http://brew.sh). as most of the other stuff is installed via brew.

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Now I install a font to use within the terminal, that works with the theme I'll select later

```bash
brew tap homebrew/cask-fonts
brew install font-hack-nerd-font
```

This is the [Powerline Hack font](https://github.com/powerline/fonts) but enhanced with font-awesome icons

Now for the Terminal itself, I like [Iterm2](https://iterm2.com) for its configurability and the fact that it can have a screen slide down from the top, much like the terminal in Quake.

After installing, I go to the profiles sections and add a new profile, where I select “Full width, top of the screen in windows -> Style and enable a hotkey (in my case double-tap ctrl)
And in both profiles, I set the font to 'hack nerd font mono' and change the dark blue colour to a lighter one, as the darker one is hardly readable on a block background

Now for the shell, I like [fish](https://fishshell.com) and by installing the [oh-my-fish](https://github.com/oh-my-fish/oh-my-fish) framework I can then add the [agnoster](https://github.com/oh-my-fish/theme-agnoster) theme.

```bash
brew install fish
```

```bash
curl https://raw.githubusercontent.com/oh-my-fish/oh-my-fish/master/bin/install | fish
```

```bash
omf install agnoster
```

Now I install lsd (ls deluxe) which gives me icons for filetypes and different shades of green depending on how old or new the files are the Hack Nerd font is needed here for the icons

```bash
brew install lsd
```

And I add the following aliases to the aliases.fish file in ~/.config/fish

```bash
alias ls='lsd'
alias ll='lsd -la'
```

Then I install [fig](https://fig.io) to have the best AI powered autocomplete there is

```bash
brew install fig
```

I also add some helper functions in fish to make life a bit easier, these are just a few examples, I have loads more

```bash
function mcd -d "make and change into directory"
    set -l dir $argv\[1\]
    mkdir -p $dir; and cd $dir
end

function reload -d "reload fish and omf"
    echo "Reloading fish config"
    source ~/.config/fish/config.fish
    echo "Reloading omf"
    omf reload
end

function urlencode -d "URL Encode any arguments"
  perl -MURI::Escape -le "print uri\_escape('$argv')"
end
```

In the end it looks like this:

<Image path="/static/images/an_opinionated_terminal_1.png" alt="terminal screenshot" thumbnail_width="600" />

Hopefully I showed you some things, that might make your commandline experience a bit more pleasurable.

Attribution: Cover Photo by [Javardh](https://unsplash.com/@_javardh_001?utm_source=medium&utm_medium=referral) on [Unsplash](https://unsplash.com?utm_source=medium&utm_medium=referral)
