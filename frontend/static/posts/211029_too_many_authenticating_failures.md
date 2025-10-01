---
title: SSH - too many authentication failures
date: "2021-10-29"
author: Gertjan Assies
tags: tips, SSH
category: tooling
image: "/static/images/sorry_were_closed.jpg"
summary: "One morning I tried to login into a remote system using ssh and suddenly I got the above error.
It took me a bit of time to figure out what the problem was.

I'm writing this here so that it might save someone some time in the future."
published: true

---
One morning I tried to login into a remote system using ssh and suddenly I got the above error.
It took me a bit of time to figure out what the problem was.

I'm writing this here so that it might save someone some time in the future.

It turned out I had too many keys added to my ssh-agent and by default, the ssh client will offer all of those to the server until one passes.

If then the number of keys added exceeds the maximum allowed authentication failures on the server, you will get this error.

There are several ways to fix this:
Firstly the IdentitiesOnly option which will only use the identities passed on the command line or in the config ignoring the ones from the agent.

```bash
> ssh -o IdentitiesOnly=yes <server>
```

This allowed my finish my task and then I started thinking of better options to manage my keys on the variety of hosts I need to connect to.

This can be easily done using the ssh-config first of all I can make the IdentitiesOnly=yes default for all hosts and then add specific keys for specific hosts, my `~/.ssh/config` now looks a little bit like this

```bash
Host server-a
  HostName server-a.company.com
  IdentityFile ~/.ssh/id_rsa_server-a
Host server-b
  HostName server-b.company.com
  IdentityFile ~/.ssh/id_rsa_server-b
Host *
  Compression yes
  IdentitiesOnly yes
  LogLevel=INFO
  PreferredAuthentications publickey
  Protocol 2
```

So no I don't need to add them to the agent anymore and I have fine-grained control over how I connect to different servers.

As ssh stops scanning after it found a match make sure the Host * is the last entry in the list.

If you want more information on all the options you can do

```bash
man ssh-config
```

or if you prefer online documentation <https://www.ssh.com/academy/ssh/config>
