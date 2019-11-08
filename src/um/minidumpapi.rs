// Copyright © 2015-2019 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Minidump API set. https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/

use shared::basetsd::ULONG64;
use shared::ntdef::{HANDLE, HRESULT, PVOID, ULONG};
use um::winnt::CONTEXT;

ENUM! {enum MINIDUMP_CALLBACK_TYPE {
    ModuleCallback,
    ThreadCallback,
    ThreadExCallback,
    IncludeThreadCallback,
    IncludeModuleCallback,
    MemoryCallback,
    CancelCallback,
    WriteKernelMinidumpCallback,
    KernelMinidumpStatusCallback,
    RemoveMemoryCallback,
    IncludeVmRegionCallback,
    IoStartCallback,
    IoWriteAllCallback,
    IoFinishCallback,
    ReadMemoryFailureCallback,
    SecondaryFlagsCallback,
    IsProcessSnapshotCallback,
    VmStartCallback,
    VmQueryCallback,
    VmPreReadCallback,
    VmPostReadCallback,
}}

STRUCT! {struct MINIDUMP_THREAD_CALLBACK {
    ThreadId: ULONG,
    ThreadHandle: HANDLE,
    Pad: ULONG,
    Context: CONTEXT,
    SizeOfContext: ULONG,
    StackBase: ULONG64,
    StackEnd: ULONG64,
}}
