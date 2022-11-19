mod profilers;
mod report;
mod interop;
mod macros;
mod utils;

#[macro_use]
extern crate log;

// Create function to list and attach profilers
register!(
    GCSurvivorsProfiler,
    ExceptionsProfiler,
    AllocationByClassProfiler,
    MemoryLeakProfiler,
    RuntimePauseProfiler,
    CpuHotpathProfiler);

static mut INVOKATIONS: u32 = 0;

// Actual COM entry point
#[no_mangle]
unsafe extern "system" fn DllGetClassObject(rclsid: ffi::REFCLSID, riid: ffi::REFIID, ppv: *mut ffi::LPVOID) -> ffi::HRESULT
{
    INVOKATIONS += 1;

    profilers::init_logging();
    info!("Successfully initialized logging");

    debug!("[profiler] Entered DllGetClassObject. Invokations: {}", INVOKATIONS);

    if ppv.is_null() {
        return ffi::E_FAIL;
    }
    
    return attach(rclsid, riid, ppv);
}