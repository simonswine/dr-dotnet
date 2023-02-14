using NUnit.Framework;
using System;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DrDotnet.Tests.Simulations;
using DrDotnet.Utils;

namespace DrDotnet.Tests.Profilers;

public class CpuHotpathProfilerTests : ProfilerTests
{
    public override Guid ProfilerGuid => new Guid("{805A308B-061C-47F3-9B30-A485B2056E71}");

    [Test]
    [Order(0)]
    [Timeout(5_000)]
    [NonParallelizable]
    public void Profiler_Exists()
    {
        Assert.NotNull(GetProfiler());
    }

    [Test, Explicit]
    [Order(1)]
    [Timeout(160_000)]
    [NonParallelizable]
    public async Task Profiler_Lists_Cpu_Hotpaths()
    {
        Logger logger = new Logger();
        SessionsDiscovery sessionsDiscovery = new SessionsDiscovery(logger);
        ProfilerMetadata profiler = GetProfiler();

        using var service1 = new FibonacciSimulation();
        using var service2 = new FibonacciSimulation();
        using var service3 = new FibonacciSimulation();
        using var service4 = new FibonacciSimulation();
        
        await Task.Delay(3000);
  
        Guid sessionId = ProfilingExtensions.StartProfilingSession(profiler, Process.GetCurrentProcess().Id, logger);

        var session = await sessionsDiscovery.AwaitUntilCompletion(sessionId);

        Console.WriteLine("Session Directory: " + session.Path);

        var summary = session.EnumerateFiles().FirstOrDefault(x => x.Name == "summary.md");

        Assert.NotNull(summary, "No summary have been created!");

        var content = File.ReadAllText(summary.FullName);

        Console.WriteLine(content);
    }
}
