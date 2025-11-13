---
title: Made a new Blog
date: "2023-06-27"
author: Gertjan Assies
summary: Learning about Svelte by making a blog
tags: sveltekit, markdown, typescript, mermaid, mdsvex
category: code
image: "/content/images/new_blog_top.jpg"
image_attribution: 'Courtesy of <a href="https://unsplash.com/s/photos/jess-bailey">Jess Bailey</a> on <a href="https://unsplash.com/">Unsplash</a>'
published: true

---

<script lang="ts">
    import Tag from '$src/components/Tag.svelte'
    import Mermaid from '$src/components/Mermaid.svelte'
</script>

## Learning experience

I wanted to learn more about creating web applications with [SvelteKit](https://kit.svelte.dev/docs/introduction) (which consists of Svelte and Vite)

so I decided to make a personal blog

so what is [Sveltekit](https://kit.svelte.dev/)? (from the website)

> SvelteKit is a framework for rapidly developing robust, performant web applications using Svelte. If you're coming from React, SvelteKit is similar to Next. If you're coming from Vue, SvelteKit is similar to Nuxt.

Sveltekit uses [Svelte](https://svelte.dev/) (an UI framework like react) and [vite](https://vitejs.dev/) (frontend tooling to help develop/run locally, with automatic reloading and caching for fast development)
and takes most of the configuration and routing away.

## Markdown

Thanks to the [mdsvex](https://mdsvex.pngwn.io/) pre-processer, I can keep writing my blogs and most pages in markdown where it is possible to embed Svelte Components in your markdown, for more interactivity.

Mdsvex will pre-process the content to first render the markdown and then pass the result on to svelte to render all the components.

for instance the content for the homepage looks like this:

```html
<script>
  import Posts from '$src/components/Posts.svelte';
</script>

# Home page

This is my personal space where I talk about technology, coding, the maker space and anything else that interests me.

## Featured blogs

<Posts tag="featured" size=3 />

## todo

...

```

## Frontmatter

Frontmatter is a convention to put metadata at the top of your markdown blog, mdsvex supports that out of the box, so when it pre-processes your markdown page, it will add a metadata field which contains all that information.

Here's an example from this blog article:

```yaml
---
title: Made a new Blog
date: "2023-06-27"
author: Gertjan Assies
summary: Learning about Svelte by making a blog
tags: sveltekit, markdown, typescript, mermaid, mdsvex, featured
category: code
image: "/content/images/new_blog_top.jpg"
published: true

---
```

## A word about routing

This is how the content of the `/src/routes` directory looks, sveltekit automatically builds routes to the pages that start with +.

```bash
.
└── src
    └── routes
        ├── +error.svelte
        ├── +layout.svelte
        ├── +page.server.ts
        ├── +page.svelte.md
        ├── about
        │   └── +page.md
        └── blog
            ├── +page.server.ts
            ├── +page.svelte
            └── [slug]
                ├── +page.server.ts
                ├── +page.svelte
                └── +page.ts
```

For instance requesting `/blog/about` from the browser will render the `/src/routes/blog/about/+page.md` page

The `+error.svelte` is returned whenever there's a 4xx/5xx response.

The `+layout.svelte` gets wrapped around all other pages.

```svelte
<!-- src/routes/+layout.svelte -->
<svelte:head>
    <title>gertjan.assies.dev</title>
</svelte:head>

<Nav />

<div class="content">
    <main in:fade>
        <slot>
            <!-- content -->
        </slot>
    </main>
</div>

<Footer />
```

where the Nav and Footer are components that will render the top and bottom part respectively and the content will replace the slot tag.

It supports parameterized paths, so whenever a request comes in for `/blog/some_blog_article`. it will go to `/blog/[slug]/+page*` with `{slug: 'some_blog_article'}` as a parameter as you can see in the code below

The order of which the files get executed is shown here, if there is a load() function in any of the .ts files, it will get executed.

<Mermaid height="50">
flowchart LR
    engine-->page.server.ts-->page.ts-->page.svelte
</Mermaid>

this also shows you you can embed [mermaid](https://mermaid.js.org/) diagrams in the blog pages

## How to render a blog article

so here's some code that gets the list of posts in this blog

```typescript
    // $lib/server/posts.ts
    type GlobEntry = {
        metadata: MetaData;
        default: unknown;
    };

    export type MetaData = {
        title: number;
        summary: string;
        date: string;
        author: string;
        tags: string;
        category: string;
        image: string;
        slug: string;
    }

    // Get all posts and add metadata
    export const posts: MetaData[] = Object.entries(
        import.meta.glob<GlobEntry>('/src/lib/posts/**/*.md', { eager: true }))
            .map(([filepath, globEntry]) => {
                return {
                ...globEntry.metadata,
                slug: parse(filepath).name,
                };
            })
            .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())

```

The meta.glob() function loads and pre-processes all the *.md files, and returns the path to the file and a globEntry which contains the parsed frontmatter metadata,

I then get the slug from the filepath, so I can link to the post itself.

As the functionality above is executed on the server (anything in $lib/server is executed serverside) it is not directly available on the client side, so when we name the file +page.server.ts it also gets executed on the server. anything returned will be passed on to the load function in +page.ts (client-side)

```typescript
// /blog/[slug]/+page.server.ts
import { posts } from '$lib/server/posts';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

// params contains any parameterized value in the url path, in this case [slug]
export const load: PageServerLoad = async ({ params }) => {
    const { slug } = params;

    // get post metadata
    const post = posts.find((post) => slug === post.slug);

    if (!post) {
        throw error(404, 'Post not found');
    }

    return {
        post,
    };
};
```

and after that the load on client is called:

```typescript
// /blog/[slug]/+page.ts
import type { PageLoad } from './$types';

// data contains what's returned from the server side load
export const load: PageLoad = async ({ data }) => {
    const component = await import(`../../../lib/posts/${data.post.slug}.md`);

    return {
        post: data.post,
        component: component.default,
    };
};
```

We get the markdown file as a component, and return it with the metadata, we call .default on the component to make sure it gets converted to commonJS to avoid things like "unexpected token: require" when svelte tries to parse it as an ES Module. this took me a while to figure out though.

```svelte
<!-- /blog/[slug]/+page.svelte -->
<script lang="ts">
    import type { PageData } from './$types';
    export let data: PageData;
</script>
...
<svelte:component this={data.component} />
...
```

which will render the post correctly, it will even render any other svelte components you put in that file for instance in this blog I can use the Tag component to show a tag here:

```svelte
<Tag path="/blog" type="category" text="code" />
```

and here it is working: <Tag path="/blog" type="category" text="code" />  click on and it will take you to list of blogs that have the code category

## Deployment

I decided to check out [render.com](https://render.com) to deploy this blog and that was by far the best and easiest experience I've had so far

All I had to do was

* Add a `Dockerfile`
* Create a new webservice in render.com
* connect the repo
* Add A and CAA records to the dns

| DNS Entries | | | | |
| :-- | -- | -- | -- | -- |
| gertjanassies.dev. | 86400 | IN | A   | 216.24.57.1 |
| gertjanassies.dev. | 86400 | IN | CAA | 0 issue "letsencrypt.org" |
| gertjanassies.dev. | 86400 | IN | CAA | 0 issuewild "letsencrypt.org" |

Render will automatically issue an SSL Certificate from the certificate authority mentioned in the CAA records and deploy the app on every commit pushed to the main branch of the repo

## What's to (or not to) love

As with most javascript/typescript frameworks, it can all too quickly becomes a bit of a mess, although during the work I learned more and more at how it all works, so I revisited working but somewhat crappy code multiple times.

So lets categorize that as a somewhat steep learning curve and my tendency to just start assembling the swedish furniture instead of reading the manual first.

I think with the modular (components) setup, thought out defaults, and ability to run the component's code on the server or client (or both) makes it a very powerful and flexible framework.

Vite making developing a pleasure with it's almost instantly refreshing pages everytime you press save. and Typescript making sure you're component properties/attributes can only hold the right stuff.

## References

* [Blog code](https://github.com/gertjana/gertjanassies.dev)
* [Sveltekit](https://kit.svelte.dev)
* Image attribution: The cover image is courtesy of [Jess Bailey](https://unsplash.com/s/photos/jess-bailey) on [unsplash](https://unsplash.com/)
