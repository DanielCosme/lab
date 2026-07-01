# Lab
Scientists learn from books, but the continuous learning happens in the lab. This repo contains experiments, projects, scripts which are used for learning, exploring and testing.

## Concurrency in Go
Concurrency, parallelism in Go.  
Practice exercises from the book: https://www.oreilly.com/library/view/concurrency-in-go/9781491941294/

## Build your own Redis
Network programming, data structures, and low-level C/C++.   
Practice execises from the book: https://build-your-own.org/redis/

## Dev Containers
Declarative developement environments using https://containers.dev/ and https://devpod.sh/

### Setup commands
```sh
devpod up . --ide zed --dotfiles https://danicos.dev/daniel/dotfiles --recreate
# set the IDE
devpod ide use zed
# set the provider
devpod provider add docker
# Set default dotfiles repo
devpod context set-options -o DOTFILES_URL=https://danicos.dev/daniel/dotfiles
```

Example with tilt and k3d: https://github.com/carlsverre/devcontainer-k3d-tilt-go
