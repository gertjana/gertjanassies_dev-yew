---
title: Implement page stats
author: Gertjan Assies
date: "2023-07-14"
category: code
tags: svelte, typescript, statistic, redis
published: true
image: "/static/images/implementing_pageviews.jpg"
summary: "Implementing Page stats starting with a PageView counter using Sveltekit and a Redis backend."
---


In a previous [blog](/blog/230627_new_blog) I talked about a blog I created with [Sveltekit](https://kit.svelte.dev)
and after I added some SEO related metadata. I wanted to see if it's actually being found and read.

I could of course just add google analytics or something similar, but I'm learning about Svelte and Typescript. so lets if we can implement a simple pageview counter, storing it in redis.
which is a superfast key/value database making it ideal for this kind of data.

[Upstash](https://upstash.com) provides one free redis db when you signup and after creating a database, you can copy paste the connection string from the dashboard.

Im using [ioredis](https://github.com/redis/ioredis), the offical redis client written in typescript;

```bash
npm install -D ioredis
```

Creating the client is a simple as creating a `new Redis(connection_string)`. In the case I populate the `REDIS_CONNECTION` env variable with the url from upstach

```typescript
import Redis from 'ioredis';

const connection = () => {
  let connection = process.env.REDIS_CONNECTION ?? "redis://localhost:6379";
  return new Redis(connection);
}
```

Then I need a function to increment the pageview

```typescript
import { dev } from '$app/environment';

export const incrementPageView: (slug: string) => Promise<number>  = async (slug: string) => {
  let prefix = "prod";
  if (dev) { prefix = "dev"; }
 return await redis().incr(`${prefix}:post:${slug}:views`);
}
```

`incr(key: string)` creates the key with value 1 if it doesn't exist, increments it otherwise and returns the updated value.

I add an prefix to the key to differentiate between my dev environment and the 'production' one. (only got one free db)

now I want a page to list all the pageviews for that I need a function that returns all those

```typescript
type PageView = {
    slug: string;
    views: number;
}

export const getPageViews: () => Promise<PageView[]> = async () => {
  let prefix = "prod";
  if (dev) { prefix = "dev"; }

  const keys = await connection().keys(`${prefix}:post:*:views`);

  if (keys.length != 0) {
    const views: string[] = (await redis().mget(...keys)).map((view) => view ?? '0');

    return keys.map((key, index) => {
        return {
            slug: key.split(':')[2],
            views: parseInt(views[index])
        }
    });
  } else {
    return [];
  }
};
```

let's go through this, Redis doesn't have functionality to return a list of key/values in one go, so first I have to get the keys

```typescript
  const keys: string[] = await connection().keys(`${prefix}post:*:views`);
```

.. and then the values, Redis has the `mget` call where you specify multiple keys as arguments. `...` is the so called spread operator which transforms an array into a list of arguments.

```typescript
  const views: PageView[] = await redis().mget(...keys).map((view) => view ?? '0');
```

when a key doesn't exists its return's `null` for that key, so the result type for mget is `Promise<(string | null)[]>` where null should never happen as i'm getting the keys from the same data. to get to a `Promise<string[]>` type I just map null's to `'0'` with the `??` operator. then I merge both the slug from the key and the views into the PageView[] result;

In the end I only have to do 2 calls.

one thing to remember is that the `keys()` method is blocking and goes over every key in your database, so for high traffic/low latency applications you are better of using `scan()` which is not blocking but only returns a subset, meaning have to call it multiple times to get all the results. I might update my code to do this, but for the purpose of this blog it's fine.

So now what's left is to call the `incrementPageviews()` method, in the server side load method that is called whenever a page is requested:

```typescript
import { incrementPageView } from '$lib/server/redis.ts';

export const load: PageServerLoad = async ({ params }) => {
  const { slug } = params;

  ...

  let pageviews = await incrementPageView(slug);

  return {
    ...,
    pageviews: pageviews,
  }
```

I also return the pageviews, so it can be displayed on the page somewhere if needed

To show all pageviews  on a page I created a component:

```svelte
<!-- /src/components/PageView.svelte -->
<script lang="ts">
  import  type { PageView } from '$lib/types';

  export let data: PageView[];
  // sort on number of page views
  let sorted: PageView[] = data.sort((a, b) => b.views - a.views);
</script>


{#if sorted.length === 0}
  <p>No pageviews yet.</p>
{:else}
  <table class="pageviews">
    <thead>
      <tr>
          <th>Page</th>
          <th>Views</th>
      </tr>
    </thead>
    <tbody>
      {#each sorted as item}
        <tr>
          <td>{item.slug}</td>
          <td class="views">{item.views}</td>
        </tr>
      {/each}
    </tbody>
  </table>
{/if}
```

and the page i'm using it in:

```typescript
// /stats/+page.server.ts
import type { PageServerLoad } from './$types';
import { getPageViews } from '$lib/server/redis.ts';

export const load: PageServerLoad = (async () => {
    return {
        pageviews: getPageViews(),
    };
  })
```

```svelte
<!-- /stats/+page.md -->
<script lang="ts">
  import PageViews from '$src/components/PageViews.svelte'

  import type { PageData } from './$types';
  export let data: PageData;
</script>

# Pageviews

<PageViews data={data.pageviews} />
```

## Further steps

I may want to implement some more functionality, calculate reading time, based on the nr of words. and then a metric to calculate the reading time.

If I do the redis data is going to move to hashsets where the slug is a hash under which several sets of metrics can be stored.
