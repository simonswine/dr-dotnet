# Dr-Dotnet 🩺

![build](https://github.com/ogxd/dr-dotnet/actions/workflows/build.yml/badge.svg)
![docker](https://github.com/ogxd/dr-dotnet/actions/workflows/docker.yml/badge.svg)

| WARNING: This project is still very WIP and may not *yet* fulfil general profiling needs! |
| --- |

## What is it

Dr-Dotnet is a set profiling tool that can be used locally or remotely to track common issues in .NET applications such as deadlocks, cpu hotpaths, zombie threads, async hotpaths (stuck tasks), memory leaks...

## Features

- **Cross platform**<br/>Dr-Dotnet can be used to profile dotnet programs running Windows, Linux or MacOS, on X86 or ARM cpus.
- **Evolutive**<br/>Dr-Dotnet isn't really "a profiler" but rather a framework for profiling. It is shipped with a suite of builtin profilers that will grow and improve hopefully thanks to the community contributions.
- **Problem focused**<br/>The spirit of the profilers shipped with Dr-Dotnet is to target a specific issue. In other words, it won't take a full dump or a deeply nested trace and let you browse it in an attempt to find a problem among the gigabytes of data generated. Despite being the way to go in theory, in real world scenarios where applications can do a lot more than hello world or give the weather, doing so is like searching for a needle in a haystack.     Instead, the approach is to propose a few profilers whose each individual function is to look for a specific problem, such as a memory leak, a deadlock, an anormal number of exceptions, a CPU hotpath, a zombie thread, ... The output of each of these analyses can in general be summarize in a couple of line or in a table, which is perfect for an human.

## How to use

There are currently 2 recommended ways to use Dr-Dotnet, depending on your usecase:

### Dr-Dotnet Desktop

This is what you want to go for if you want to profile a dotnet program locally.    
⚠ At the moment, Dr-Dotnet Desktop is based on WPF and thus it is Windows only, however, as it is just running a browser to display the web app version, it shouldn't be too hard to do this in a cross-platform way with something like Avalonia in a near future.

### Dr-Dotnet as a Docker Sidecar

There is currently a CI step to build a docker image available at `ghcr.io/ogxd/drdotnet:latest`.    
This image can run on a host as a docker container, next to the container you want to profile.    
The container you want to profile must be running a dotnet program (of course) and be ran with a few flags to allow Dr-Dotnet to attach/detach later, if needs be.    
<pre>docker run --rm -it --name **YOUR_APP_NAME** -v /tmp:/tmp --ipc="shareable" **YOUR_IMAGE**</pre>
Then, you are ready to start Dr-Dotnet:
<pre>docker run --rm -it --name drdotnet -v /tmp:/tmp --ipc="container:**YOUR_APP_NAME**" --pid="container:myapp" -p 8000:92 ghcr.io/ogxd/drdotnet:latest</pre>
You can run Dr-Dotnet anytime you want, or leave it running all the time, it won't do anything if you don't use it. Make sure the port is private to your network however for security reasons, you don't want your profiler to be open to the public ;)

## How to contribute

At the moment, the project is still fairly new. Hence, there is *yet* no proper versionning, roadmap nor strict contribution guidelines. Any contribution is welcome, as long as it remains in the spirit of the project. In short, the idea is to:
- Cover the most common issues (find deadlocks, detect memoryleaks, list CPU hotpaths, ...)
- Be relatively easy to use (the output should be as concise as possible, this is not easy)
- And have as little overhead as possible (if the program is altered too much, you may have a biaised analysis)

### How it works

The .NET Profiling API is accessible via COM interop (cross-platform thanks to the Platform Adaptation Layer) and allows little overheap profiling compared to other methods while giving a wide range of possibilities for profiling purpose. Perfview uses this API (on top of ETW) however it does it from managed code calling mocked COM objects written in C#.     
In this project, we're using Rust for coding the profilers for the safety and the modernity of the language. The CLR profiling API rust binding are originally taken from [this project from Camden Reslink](https://github.com/camdenreslink/clr-profiler) who did a really great job.    
The UI and profilers management are coded in C#, for the productivity the language offers and because it is convenient for interperability with Rust. Bindings between C# and Rust are autogenerated using [FFIDJI](https://github.com/ogxd/ffidji) (but they are fairly trivial for now, to be honest this is probably overkill).

### How to build

#### Prerequisites

- .NET SDK 6.0
- Rust toolchain
- I guess that's it :)

#### Building

Build either `DrDotnet.Web.csproj`, `DrDotnet.Desktop.csproj`, or the solution `DrDotnet.sln`, depending on how you plan to use Dr-Dotnet.
The `DrDotnet.csproj` project has a prebuild step that will try to build the Rust profilers. If it fails, you'll find the Rust compiler output in the Output window for instance if you are using Visual Studio. You usually don't need to use cargo commands yourself directly at this first stage.

#### Creating new profilers

The `DrDotnet.csproj` links to the Rust part, however, if you really get into writing Rust for this project, it's probably better to edit the Rust part from another IDE than Visual Studio or Rider (I use VS Code with Rust extensions).   
The Rust code base is divider in two projects:
- `profilers` is where all profilers are
- `profiling_api` is where is CLR profiling API bindings are (it wraps the bizarre C syntax and brings some safety and convenience to it)

To create a new profiler, checkout `src/DrDotnet.Profilers/profilers/src/profilers/` and checkout any profiler. You can basically duplicate one and change its UUID (make it unique). Then, head to `src/DrDotnet.Profilers/profilers/src/lib.rs/` and add the new profiler in the `register!` macro. Then you're good to go, you can now start using the CLR Profiling API. Checkout [the official documentation to get started](https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/profiling-interfaces). I also recommend checking our [Christophe Nasarre blog posts](https://chnasarre.medium.com/start-a-journey-into-the-net-profiling-apis-40c76e2e36cc) for a more "friendly" introduction to this API ;)    
Note: You'll need to set `is_released` in your profiler to true if you want to be able to view your profiler in the C# UI when built in release mode.    
Another note: DrDotnet attaches to an already running process, meaning that not all callbacks are usable, only those who can be enable after attaching. See the flags [here](https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/cor-prf-monitor-enumeration) and [here](https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/profiling/cor-prf-high-monitor-enumeration).

### Useful Links

- [Pavel Yosifovich — Writing a .NET Core cross platform profiler in an hour](https://www.youtube.com/watch?v=TqS4OEWn6hQ)
- [Pavel Yosifovich — DotNext Moscou 2019 Source Code](https://github.com/zodiacon/DotNextMoscow2019)
- [Josef Biehler - Create a .NET profiler with the Profiling API](https://dev.to/gabbersepp/create-a-net-profiler-with-the-profiling-api-start-of-an-unexpected-journey-198n)
- [Mattias Högström - Building a Mixed-Mode Sampling Profiler](https://www.codeproject.com/Articles/384514/Building-a-Mixed-Mode-Sampling-Profiler)
- [Christophe Nasarre - Start a journey into the .NET Profiling APIs](https://chnasarre.medium.com/start-a-journey-into-the-net-profiling-apis-40c76e2e36cc)
- [Some random COM C++ source code](https://github.com/tenable/poc/blob/master/Comodo/Comodo%20Antivirus/ComodoInjectionCode/ComodoInjectionCode/InjectedCode.cpp)
- [Some random COM C++ source code](https://cpp.hotexamples.com/examples/-/ICLRRuntimeInfo/GetInterface/cpp-iclrruntimeinfo-getinterface-method-examples.html)
