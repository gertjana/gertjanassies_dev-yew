---
title: Vibe Coding - an experiment
author: Gertjan Assies
date: "2025-04-29"
category: code
tags: rust, ai, vibecoding, featured
published: false
image: ""
summary: "Does AI make me more productive?"
---

<script lang="ts">
    import { Lightbox } from 'svelte-lightbox';
</script>

For a long time I wanted to write an application where I could just drop a note, a link to a post or article, an image etc. as one action, and then later being able to retrieve it.

With the advance of AI, I thought about using AI to classify the content, say put some tags on ot, and then later use those tags to query the content.

So i took two approaches

    * Developer role: Write it myself using CoPilot for autocompletion only
    * Vibe Coding: just tell the AI what I want. but more granular, asking it to implement small features/tests/documentation

 The requirements:

    * API based backend in Rust
    * Pluggable AI Agents (Claude/ChatGPT)
    * Pluggable storage for content (Redis/S3, filesystem)
    * Pluggable storage for tags (Redis)

So this is what I did using cursor.ai with the Claude Sonnet 3.7 model, the details of the application is for another blog article, but here I will focus on the coding approaches

Timewise, working in the evenings/weekends, It took around 30 hours to get a working prototype, and around 6 hours to get the Vibe Coding version up and running.
So an increase of 5x in productivity.

I noticed that if you make the requests to big and/or too vague, you spend a lot of time correcting the AI. and sometimes even have to revert back to an earlier version.

Keeping your requests small and detailed, it's pretty good. but you have multiple roles

    * (Technical) Product Owner: you have to know what you want, and how to ask it
    * Reviewer: Be as critical as possible, and don't trust the AI
    * Developer: Sometimes it's quicker to make the adjustments yourself

It does make mistakes though and n


So in conclusion, AI will not replace developers, but it will make them more productive. It will also make it easier for non-developers to create applications, but they still will need to know what they want and how to ask for it, and understand basic development principles like version control, testing, etc. and ba able to judge architectual decisions, data structures used etc.

I haven't ran into any hallucations, but maybe that's because I kept my requests small and consise. but on more then one occasion it got into an infinite loop, for instance adding the same dependency over and over again. 

It also did not put a newline at the end of newly created files, and only did it after I asked it to remember to do that always.

It's another tool in the software engineer's toolbox, and with every tool, you have to learn how to use it.

So now I have another use for AI after:

    * Regex / JQ and AWS CLI Queries
    * Corporate BS

#@ References

* [The Code](https://github.com/gertjana/ai_classify)
* [Cursor.ai](https://cursor.ai/)
* [Claude AI](https://claude.ai/)
* [ChatGPT](https://openai.com/)
* [Rust](https://www.rust-lang.org/)
