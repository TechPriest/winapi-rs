// Copyright © 2015-2019 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Minidump API set. https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/

use shared::basetsd::ULONG64;
use shared::minwindef::DWORD;
use shared::ntdef::{HANDLE, HRESULT, PVOID, PWCHAR, ULONG};
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
pub type PMINIDUMP_THREAD_CALLBACK = *mut MINIDUMP_THREAD_CALLBACK;

STRUCT! {struct MINIDUMP_THREAD_EX_CALLBACK {
    ThreadId: ULONG,
    ThreadHandle: HANDLE,
    Pad: ULONG,
    Context: CONTEXT,
    SizeOfContext: ULONG,
    StackBase: ULONG64,
    StackEnd: ULONG64,
    BackingStoreBase: ULONG64,
    BackingStoreEnd: ULONG64,
}}
pub type PMINIDUMP_THREAD_EX_CALLBACK = *mut MINIDUMP_THREAD_EX_CALLBACK;

STRUCT! {struct VS_FIXEDFILEINFO {
    dwSignature: DWORD,
    dwStrucVersion: DWORD,
    dwFileVersionMS: DWORD,
    dwFileVersionLS: DWORD,
    dwProductVersionMS: DWORD,
    dwProductVersionLS: DWORD,
    dwFileFlagsMask: DWORD,
    dwFileFlags: DWORD,
    dwFileOS: DWORD,
    dwFileType: DWORD,
    dwFileSubtype: DWORD,
    dwFileDateMS: DWORD,
    dwFileDateLS: DWORD,
}}

STRUCT! {struct MINIDUMP_MODULE_CALLBACK {
    FullPath: PWCHAR,
    BaseOfImage: ULONG64,
    SizeOfImage: ULONG,
    CheckSum: ULONG,
    TimeDateStamp: ULONG,
    VersionInfo: VS_FIXEDFILEINFO,
    CvRecord: PVOID,
    SizeOfCvRecord: ULONG,
    MiscRecord: PVOID,
    SizeOfMiscRecord: ULONG,
}}
pub type PMINIDUMP_MODULE_CALLBACK = *mut MINIDUMP_MODULE_CALLBACK;

STRUCT! {struct MINIDUMP_INCLUDE_THREAD_CALLBACK {
    ThreadId: ULONG,
}}
pub type PMINIDUMP_INCLUDE_THREAD_CALLBACK = *mut MINIDUMP_INCLUDE_THREAD_CALLBACK;

STRUCT! {struct MINIDUMP_INCLUDE_MODULE_CALLBACK {
    BaseOfImage: ULONG64,
}}
pub type PMINIDUMP_INCLUDE_MODULE_CALLBACK = *mut MINIDUMP_INCLUDE_MODULE_CALLBACK;

STRUCT! {struct MINIDUMP_IO_CALLBACK {
    Handle: HANDLE,
    Offset: ULONG64,
    Buffer: PVOID,
    BufferBytes: ULONG,
}}
pub type PMINIDUMP_IO_CALLBACK = *mut MINIDUMP_IO_CALLBACK;

STRUCT! {struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK {
    Offset: ULONG64,
    Bytes: ULONG,
    FailureStatus: HRESULT,
}}
pub type PMINIDUMP_READ_MEMORY_FAILURE_CALLBACK = *mut MINIDUMP_READ_MEMORY_FAILURE_CALLBACK;

ENUM! {enum MINIDUMP_SECONDARY_FLAGS {
    MiniSecondaryWithoutPowerInfo,
    MiniSecondaryValidFlags,
}}

STRUCT! {struct MINIDUMP_VM_QUERY_CALLBACK {
    Offset: ULONG64,
}}
pub type PMINIDUMP_VM_QUERY_CALLBACK = *mut MINIDUMP_VM_QUERY_CALLBACK;

STRUCT! {struct MINIDUMP_VM_PRE_READ_CALLBACK {
    Offset: ULONG64,
    Buffer: PVOID,
    Size: ULONG,
}}
pub type PMINIDUMP_VM_PRE_READ_CALLBACK = *mut MINIDUMP_VM_PRE_READ_CALLBACK;

STRUCT! {struct MINIDUMP_VM_POST_READ_CALLBACK{
    Offset: ULONG64,
    Buffer: PVOID,
    Size: ULONG,
    Completed: ULONG,
    Status: HRESULT,
}}
pub type PMINIDUMP_VM_POST_READ_CALLBACK = *mut MINIDUMP_VM_POST_READ_CALLBACK;

UNION! {union MINIDUMP_CALLBACK_INPUT_u {
    [u32; 324],
    ProcessId ProcessId_mut: ULONG,
    Thread Thread_mut: MINIDUMP_THREAD_CALLBACK,
    ThreadEx ThreadEx_mut: MINIDUMP_THREAD_EX_CALLBACK,
    Module Module_mut: MINIDUMP_MODULE_CALLBACK,
    IncludeThread IncludeThread_mut: MINIDUMP_INCLUDE_THREAD_CALLBACK,
    IncludeModule IncludeModule_mut: MINIDUMP_INCLUDE_MODULE_CALLBACK,
    Io Io_mut: MINIDUMP_IO_CALLBACK,
    ReadMemoryFailure ReadMemoryFailure_mut: MINIDUMP_READ_MEMORY_FAILURE_CALLBACK,
    SecondaryFlags SecondaryFlags_mut: ULONG,
    VmQuery VmQuery_mut: MINIDUMP_VM_QUERY_CALLBACK,
    VmPreRead VmPreRead_mut: MINIDUMP_VM_PRE_READ_CALLBACK,
    VmPostRead VmPostRead_mut: MINIDUMP_VM_POST_READ_CALLBACK,
}}
