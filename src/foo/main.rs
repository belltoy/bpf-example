#![no_std]
#![no_main]

// use cty::*;

// use one of the preludes
// use redbpf_probes::kprobe::prelude::*;
use redbpf_probes::xdp::prelude::*;
// use redbpf_probes::socket_filter::prelude::*;


// Use the types you're going to share with userspace, eg:
// use bpf_example::foo::SomeEvent;

program!(0xFFFFFFFE, "GPL");

// The maps and probe function go here, eg:
//
// #[map("syscall_events")]
// static mut syscall_events: PerfMap<SomeEvent> = PrefMap::with_max_entries(1024);
//
// #[kprobe("__x64_sys_open")]
// fn syscall_enter(regs: Registers) {
//     let pid_tgid = bpf_get_current_pid_tgid();
//     ...
//
//     let event = SomeEvent {
//     pid: pid_tgid >> 32,
//     ...
//     };
//     unsafe { syscall_events.insert(regs.ctx, &event) };
// }

#[xdp]
pub fn block_port_80(ctx: XdpContext) -> XdpResult {
    if let Ok(transport) = ctx.transport() {
        if transport.dest() == 80 {
            return Ok(XdpAction::Drop);
        }
    }

    Ok(XdpAction::Pass)
}
