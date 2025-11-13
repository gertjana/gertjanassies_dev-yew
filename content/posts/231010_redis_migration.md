---
title: Redis Migration
author: Gertjan Assies
date: "2023-10-10"
category: code
tags: redis, dragonflydb, tip
published: true
image: "/content/images/migration.jpg"
summary: "Migrating from redis to dragonflydb for this blog."
---

I've read about [dragonflydb](https://dragonflydb.io):

> Dragonfly is a drop-in Redis® replacement that is optimized for data-intensive, low latency applications. Applications built on Dragonfly get the full speed, reliability, and scalability that modern cloud hardware makes possible, allowing them to deliver incredible experiences to their users while reducing both costs and complexity.

so reading between all the marketing mumbo jumbo, it sounds like it's really fast and could replace any redis instance.

So as a small exercise  I decided to migrate this blog which uses redis to dragonflydb.

For the migration I followed the practice of first continously replicating the data, and then switch the application to the new one, and it turned out to be pretty easy:

Prerequisites: the old `redis-instance` and a fresh `dragonfly-instance` one and `redis-cli` installed.

```bash
> redis-cli -h <dragonfly-instance> -p 6379

# an empty db
dragonfly-instance:6379> KEYS *
(empty array)

# with no replication
dragonfly-instance:6379> INFO replication
# Replication
role:master
connected_slaves:0
master_replid:68ffe35f52d262f9e86b7e14de8ee98020764bb6

# turn on replication
dragonfly-instance:6379> REPLICAOF <old redis-instance> 6379
OK

dragonfly-instance:6379> INFO replication
# Replication
role:replica
master_host:<old redis-instance>
master_port:6379
master_link_status:up
master_last_io_seconds_ago:1
master_sync_in_progress:0

# we now have all the data here
dragonfly-instance:6379> KEYS *
# shows all the migrated keys, but for obvious reasons I've omitted them
```

Now I stop and update the server to use the new instance

now I turn off replication, this is needed as replica's are read only.

```bash
dragonfly-instance:6379> REPLICAOF NO ONE
OK
```

And now start the application again.

And that's it, the blog is now using dragonflydb instead of redis.

Now all is needed is to get those millions and millions of visitors to this blog. so i can reap the benefits of the speed of dragonflydb. /s
