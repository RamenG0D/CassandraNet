/// Copyright (C) 2024-present WD Studios L.L.C. & CassandraNet Contributors.


pub struct CnProfile;

// there might be a better way to do this, but this is likely flexible enough
pub trait Profiler {
    /// Start a new event with the given name
    fn start_event(event_name: &'static str);
    /// End the current event
    fn end_event();
    /// Set the name of the current thread
    fn set_thread_name(thread_name: &str);
    /// Start a new event with the given name and data
    fn start_event_with_data(event_name: &'static str, data: &str, color: u32);
    /// Unregister a fiber
    fn unregister_fiber(in_fiber_id: u64);
    /// Begin a fiber switch
    fn begin_fiber_switch(in_current_fiber_id: u64, in_new_fiber_id: u64);
    /// End a fiber switch
    fn end_fiber_switch(in_fiber_id: u64);
}

/// Public interface for profiling with CassandraNet. This is a wrapper around the superluminal_perf crate. if you want to use a different profiler, you can implement this trait for that profiler.
impl Profiler for CnProfile {
    fn start_event(event_name: &'static str) {
        superluminal_perf::begin_event(event_name);
    }

    fn end_event() {
        superluminal_perf::end_event();
    }

    fn set_thread_name(thread_name: &str) {
        superluminal_perf::set_current_thread_name(thread_name);
    }

    fn start_event_with_data(event_name: &'static str, data: &str, color: u32) {
        superluminal_perf::begin_event_with_data(event_name, data, color);
    }

    fn unregister_fiber(in_fiber_id: u64) {
        superluminal_perf::unregister_fiber(in_fiber_id);
    }

    fn begin_fiber_switch(in_current_fiber_id: u64, in_new_fiber_id: u64) {
        superluminal_perf::begin_fiber_switch(in_current_fiber_id, in_new_fiber_id);
    }

    fn end_fiber_switch(in_fiber_id: u64) {
        superluminal_perf::end_fiber_switch(in_fiber_id);
    }
}
