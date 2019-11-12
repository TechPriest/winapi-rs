// Copyright © 2015-2019 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Minidump API set. https://docs.microsoft.com/en-us/windows/win32/api/minidumpapiset/

use shared::basetsd::{ULONG32, ULONG64};
use shared::minwindef::{BOOL, DWORD};
use shared::ntdef::{HANDLE, HRESULT, PVOID, PWCHAR, ULONG};
use um::winnt::{CONTEXT, PEXCEPTION_POINTERS};

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

STRUCT! {struct MINIDUMP_CALLBACK_INPUT {
    ProcessId: ULONG,
    ProcessHandle: HANDLE,
    CallbackType: ULONG,
    u: MINIDUMP_CALLBACK_INPUT_u,
}}
pub type PMINIDUMP_CALLBACK_INPUT = *mut MINIDUMP_CALLBACK_INPUT;

STRUCT! {struct MINIDUMP_MEMORY_INFO {
    BaseAddress: ULONG64,
    AllocationBase: ULONG64,
    AllocationProtect: ULONG32,
    __alignment1: ULONG32,
    RegionSize: ULONG64,
    State: ULONG32,
    Protect: ULONG32,
    Type: ULONG32,
    __alignment2: ULONG32,
}}
pub type PMINIDUMP_MEMORY_INFO = *mut MINIDUMP_MEMORY_INFO;

STRUCT! {struct MINIDUMP_CALLBACK_OUTPUT_Memory {
    MemoryBase: ULONG64,
    MemorySize: ULONG,
}}

STRUCT! {struct MINIDUMP_CALLBACK_OUTPUT_Cancel {
    CheckCancel: BOOL,
    Cancel: BOOL,
}}

STRUCT! {struct MINIDUMP_CALLBACK_OUTPUT_VmRegion {
    VmRegion: MINIDUMP_MEMORY_INFO,
    Continue: BOOL,
}}

STRUCT! {struct MINIDUMP_CALLBACK_OUTPUT_VmQueryStatus {
    VmQueryStatus: HRESULT,
    VmQueryResult: MINIDUMP_MEMORY_INFO,
}}

STRUCT! {struct MINIDUMP_CALLBACK_OUTPUT_VmReadStatus {
    VmReadStatus: HRESULT,
    VmReadBytesCompleted: ULONG,
}}

UNION! {union MINIDUMP_CALLBACK_OUTPUT {
    [u32;13],
    ModuleWriteFlags: ULONG,
    ThreadWriteFlags: ULONG,
    SecondaryFlags: ULONG,
    Memory: MINIDUMP_CALLBACK_OUTPUT_Memory,
    Cancel: MINIDUMP_CALLBACK_OUTPUT_Cancel,
    Handle: HANDLE,
    VmRegion: MINIDUMP_CALLBACK_OUTPUT_VmRegion,
    VmQueryStatus: MINIDUMP_CALLBACK_OUTPUT_VmQueryStatus,
    VmReadStatus: MINIDUMP_CALLBACK_OUTPUT_VmReadStatus,
    Status: HRESULT,
}}
pub type PMINIDUMP_CALLBACK_OUTPUT = *mut MINIDUMP_CALLBACK_OUTPUT;

ENUM! {enum MINIDUMP_TYPE {
    MiniDumpNormal,
    MiniDumpWithDataSegs,
    MiniDumpWithFullMemory,
    MiniDumpWithHandleData,
    MiniDumpFilterMemory,
    MiniDumpScanMemory,
    MiniDumpWithUnloadedModules,
    MiniDumpWithIndirectlyReferencedMemory,
    MiniDumpFilterModulePaths,
    MiniDumpWithProcessThreadData,
    MiniDumpWithPrivateReadWriteMemory,
    MiniDumpWithoutOptionalData,
    MiniDumpWithFullMemoryInfo,
    MiniDumpWithThreadInfo,
    MiniDumpWithCodeSegs,
    MiniDumpWithoutAuxiliaryState,
    MiniDumpWithFullAuxiliaryState,
    MiniDumpWithPrivateWriteCopyMemory,
    MiniDumpIgnoreInaccessibleMemory,
    MiniDumpWithTokenInformation,
    MiniDumpWithModuleHeaders,
    MiniDumpFilterTriage,
    MiniDumpWithAvxXStateContext,
    MiniDumpWithIptTrace,
    MiniDumpValidTypeFlags,
    MiniDumpScanInaccessiblePartialPages,
}}

STRUCT! {struct MINIDUMP_EXCEPTION_INFORMATION {
    ThreadId: DWORD,
    ExceptionPointers: PEXCEPTION_POINTERS,
    ClientPointers: BOOL,
}}
pub type PMINIDUMP_EXCEPTION_INFORMATION = *mut MINIDUMP_EXCEPTION_INFORMATION;

ENUM! {enum MINIDUMP_STREAM_TYPE {
    UnusedStream,
    ReservedStream0,
    ReservedStream1,
    ThreadListStream,
    ModuleListStream,
    MemoryListStream,
    ExceptionStream,
    SystemInfoStream,
    ThreadExListStream,
    Memory64ListStream,
    CommentStreamA,
    CommentStreamW,
    HandleDataStream,
    FunctionTableStream,
    UnloadedModuleListStream,
    MiscInfoStream,
    MemoryInfoListStream,
    ThreadInfoListStream,
    HandleOperationListStream,
    TokenStream,
    JavaScriptDataStream,
    SystemMemoryInfoStream,
    ProcessVmCountersStream,
    IptTraceStream,
    ThreadNamesStream,
    ceStreamNull,
    ceStreamSystemInfo,
    ceStreamException,
    ceStreamModuleList,
    ceStreamProcessList,
    ceStreamThreadList,
    ceStreamThreadContextList,
    ceStreamThreadCallStackList,
    ceStreamMemoryVirtualList,
    ceStreamMemoryPhysicalList,
    ceStreamBucketParameters,
    ceStreamProcessModuleMap,
    ceStreamDiagnosisList,
    LastReservedStream,
}}

STRUCT! {struct MINIDUMP_USER_STREAM {
    Type: ULONG32,
    BufferSize: ULONG,
    Buffer: PVOID,
}}
pub type PMINIDUMP_USER_STREAM = *mut MINIDUMP_USER_STREAM;

STRUCT! {struct MINIDUMP_USER_STREAM_INFORMATION {
    UserStreamCount: ULONG,
    UserStreamArray: PMINIDUMP_USER_STREAM,
}}
pub type PMINIDUMP_USER_STREAM_INFORMATION = *mut MINIDUMP_USER_STREAM_INFORMATION;

pub type MINIDUMP_CALLBACK_ROUTINE = extern "system" fn(CallbackParam: PVOID, 
    CallbackInput: PMINIDUMP_CALLBACK_INPUT,
    CallbackOutput: PMINIDUMP_CALLBACK_OUTPUT
);

STRUCT! {struct MINIDUMP_CALLBACK_INFORMATION {
    CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    CallbackParam: PVOID,
}}
pub type PMINIDUMP_CALLBACK_INFORMATION = *mut MINIDUMP_CALLBACK_INFORMATION;


extern "system" {
    pub fn MiniDumpWriteDump(hProcess: HANDLE,
        ProcessId: DWORD,
        hFile: HANDLE,
        DumpType: MINIDUMP_TYPE,
        ExceptionParam: PMINIDUMP_EXCEPTION_INFORMATION,
        UserStreamParam: PMINIDUMP_USER_STREAM_INFORMATION,
        CallbackParam: PMINIDUMP_CALLBACK_INFORMATION
    ) -> BOOL;

    /*pub fn MiniDumpReadDumpStream(
        BaseOfDump: PVOID,
        StreamNumber: ULONG,
        Dir: PMINIDUMP_DIRECTORY,
        StreamPointer: *mut PVOID,
        StreamSize: *mut USIZE,
    ) -> BOOL;*/
}
