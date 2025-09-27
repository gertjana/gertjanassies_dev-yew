---
title: Simple AI prompt on the commandline
author: Gertjan Assies
date: "2024-08-30"
category: code
tags: generativeai, llm, python, cmd
published: false
image: ""
summary: "how to create an AI prompt on the commandline"
---

<script lang="ts">
    import { Lightbox } from 'svelte-lightbox';
</script>


```
 > env GEMINI_API_KEY=<secret> uv run main.py
You: Write a Haiku about using Generative AI on the commandline
Prompt in the dark,
AI weaves words on the screen,
Code sings a new song. 
```






```
import google.generativeai as genai
import os

def main():
    api_key = os.environ['GEMINI_API_KEY']
    genai.configure(api_key=api_key)
    model = genai.GenerativeModel("gemini-1.5-flash")
    chat = model.start_chat()
    while True:
        user_input = input("You: ")
        response = chat.send_message(user_input)
        print(response.text)
        
if __name__ == "__main__":
    main()
```