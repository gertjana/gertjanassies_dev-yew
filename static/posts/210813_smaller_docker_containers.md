---
title: Smaller docker containers
author: Gertjan Assies
date: "2022-08-13"
category: code
tags: docker, go, scratch, optimization
image: "/static/images/smaller_docker_containers_top.jpg"
summary: How to create your docker containers as small as possible
published: true

---

In the company where I work as a Site Reliability Engineer, we currently have around 60k devices in the field, we are expected to scale that to half a million by 2025. and they all need to communicate with our back-office which consists of a whole bunch of containerized microservices.  
This means for everything that we do, we have to ask ourselves the age-old question:

> does it scale?

and when it does, how can we make sure the costs do not scale faster than the resources we need.  
One thing to help with that is to make sure our docker containers are as small as possible.  
This will also help in other areas, less software running means less things can crash and a significantly smaller attack surface, so stability and security will increase too, Hoorah!

The example I'm going to expand upon works with a small go application but could be applied to almost any language that compiles to a native application.

If you want to dive into the code first, here's the repo: [https://gitlab.com/gertjana/tinyweb](https://gitlab.com/gertjana/tinyweb)

## Something to test with

The infamous hello world application here is done with the [echo](https://echo.labstack.com/) web framework just to have something I can verify working from within the container.

```go
package main
import (  
  "net/http"  
  "github.com/labstack/echo/v4"  
  "github.com/labstack/echo/v4/middleware"  
)
func main() {  
  e := echo.New()  
  e.Use(middleware.Logger())  
  e.GET("/", hello)  
  e.Logger.Fatal(e.Start(":8000"))  
}

func hello(c echo.Context) error {  
  return c.String(http.StatusOK, "Hello, World!")  
}
```

## Build a normal docker container for your app

```docker
FROM golang:1.16
RUN mkdir -p /app  
COPY main.go /app  
COPY go.\* /app  
WORKDIR /app
RUN CGO\_ENABLED=0 GOOS=linux go build -o service main.go
EXPOSE 8000  
CMD \["/app/service"\]
```

To set a baseline, I'm just creating a docker image from the golang:1.16 base-image, copy over the sources and compile them.

> Image size: 968 Mb (968212150) almost a Gb!

## A multistage docker image

```docker
FROM golang:1.16 AS builderCOPY main.go /app  
COPY go.\* /app  
WORKDIR /app
RUN CGO\_ENABLED=0 GOOS=linux go build -o dist/service main.go
ADD https://github.com/krallin/tini/releases/download/v0.19.0/tini-static /tini  
RUN chmod +x /tini

FROM scratch
COPY --from=builder /app/dist/service /service  
COPY --from=builder /tini /tini
EXPOSE 8000
ENTRYPOINT ["/tini", "--"]  
CMD ["/service"]
```

Here I'm doing a multistage build, still using the Golang image to build the application but then in the second stage, I copy over the compiled application to a base image called scratch.  
[Scratch](https://hub.docker.com/_/scratch/) is a special base image which you cannot pull or download, but it is basically a completely empty container to start with.

As it is empty it also does not contain an init system which normally runs on Linux systems and serves as the root process (PID 1) for all other processes, making sure the right signals are sent to the right process, zombie processes are cleared up etc.  
[Tini](https://github.com/krallin/tini) is a small init system for containers. this way the application thinks it's running in a ‘normal' Linux system.

> Image size: 8.8 Mb (8799493)

## Remove debug info

with the -s -w ldflags you can tell the compiler to not put any debug information in the binary. (for production systems it would be prudent to also create an image with the debug information in case you need to fix an urgent issue)

The Docker file is the same as above except for the following line

```docker
RUN CGO\_ENABLED=0 GOOS=linux go build **\-ldflags="-s -w"** -o dist/service main.go
```

> Image size: 6.5 Mb (6559840)

## Compress binary with UPX

[UPX](https://upx.github.io/) is a tool that compresses executables.

after the compile step in the build stage, the following will download and execute upx on the freshly build application.

```docker
ADD https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64\_linux.tar.xz upx-3.96-amd64\_linux.tar.xz  
RUN tar -xf upx-3.96-amd64\_linux.tar.xz  
RUN mv upx-3.96-amd64\_linux/upx .  
RUN ./upx --brute /app/dist/service
```

> Resulting Image size: 2.6 Mb (2567632)

## Conclusion

to summarize the images created with their sizes

| Image | Debug info| UPX compression | Size | Percentage |
| -- | :--: | :--: | -- | -- |
| golang  | yes | no  | 968212150 | 100.00%
| scratch | yes | no  | 8799493   | 0.91%
| scratch | no  | no  | 6559840   | 0.68%
| scratch | no  | yes | 2567632   | 0.27%

We managed to get the image size down to 0.27% of the original which is a lot! Most of the gain was in using the scratch image, which avoided having a complete Linux system installed there.  
Multi-stage builds make it possible to have an image with all the tools needed to build your app and then just copy over the built application to an image that only has the minimum needed to run your application.

So I hope I've shown you to always try to create the minimal image you need to run your application.

I used the following to determine the image sizes:

```bash
> docker inspect -f "{{ .Size }}" gertjana/tinyweb:tiniest  
2567632
```

## References

* [https://gitlab.com/gertjana/tinyweb](https://gitlab.com/gertjana/tinyweb)  
    where all the shown code lives
* [https://hub.docker.com/\_/scratch/](https://hub.docker.com/_/scratch/)  
    Info about the scratch base-image
* [https://echo.labstack.com/](https://echo.labstack.com/)  
    minimalistic high-performance web framework
* [https://github.com/krallin/tini](https://github.com/krallin/tini)  
    a small but valid ini t system
* [https://upx.github.io/](https://upx.github.io/)  
    a free, portable, extendable, high-performance executable packer

## Image attribution

courtesy [https://unsplash.com/@guibolduc](https://unsplash.com/@guibolduc)
