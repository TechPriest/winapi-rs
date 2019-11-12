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
    MiniSecondaryWithoutPowerInfo = 0x00000001,
    MiniSecondaryValidFlags       = 0x00000001,
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
    [u32; 13],
    ModuleWriteFlags ModuleWriteFlags_mut: ULONG,
    ThreadWriteFlags ThreadWriteFlags_mut: ULONG,
    SecondaryFlags SecondaryFlags_mut: ULONG,
    Memory Memory_mut: MINIDUMP_CALLBACK_OUTPUT_Memory,
    Cancel Cancel_mut: MINIDUMP_CALLBACK_OUTPUT_Cancel,
    Handle Handle_mut: HANDLE,
    VmRegion VmRegion_mut: MINIDUMP_CALLBACK_OUTPUT_VmRegion,
    VmQueryStatus VmQueryStatus_mut: MINIDUMP_CALLBACK_OUTPUT_VmQueryStatus,
    VmReadStatus VmReadStatus_mut: MINIDUMP_CALLBACK_OUTPUT_VmReadStatus,
    Status Status_mut: HRESULT,
}}
pub type PMINIDUMP_CALLBACK_OUTPUT = *mut MINIDUMP_CALLBACK_OUTPUT;

ENUM! {enum MINIDUMP_TYPE {
    MiniDumpNormal                         = 0x00000000,
    MiniDumpWithDataSegs                   = 0x00000001,
    MiniDumpWithFullMemory                 = 0x00000002,
    MiniDumpWithHandleData                 = 0x00000004,
    MiniDumpFilterMemory                   = 0x00000008,
    MiniDumpScanMemory                     = 0x00000010,
    MiniDumpWithUnloadedModules            = 0x00000020,
    MiniDumpWithIndirectlyReferencedMemory = 0x00000040,
    MiniDumpFilterModulePaths              = 0x00000080,
    MiniDumpWithProcessThreadData          = 0x00000100,
    MiniDumpWithPrivateReadWriteMemory     = 0x00000200,
    MiniDumpWithoutOptionalData            = 0x00000400,
    MiniDumpWithFullMemoryInfo             = 0x00000800,
    MiniDumpWithThreadInfo                 = 0x00001000,
    MiniDumpWithCodeSegs                   = 0x00002000,
    MiniDumpWithoutAuxiliaryState          = 0x00004000,
    MiniDumpWithFullAuxiliaryState         = 0x00008000,
    MiniDumpWithPrivateWriteCopyMemory     = 0x00010000,
    MiniDumpIgnoreInaccessibleMemory       = 0x00020000,
    MiniDumpWithTokenInformation           = 0x00040000,
    MiniDumpWithModuleHeaders              = 0x00080000,
    MiniDumpFilterTriage                   = 0x00100000,
    MiniDumpWithAvxXStateContext           = 0x00200000,
    MiniDumpWithIptTrace                   = 0x00400000,
    MiniDumpScanInaccessiblePartialPages   = 0x00800000,
    MiniDumpValidTypeFlags                 = 0x00ffffff,
}}

STRUCT! {struct MINIDUMP_EXCEPTION_INFORMATION {
    ThreadId: DWORD,
    ExceptionPointers: PEXCEPTION_POINTERS,
    ClientPointers: BOOL,
}}
pub type PMINIDUMP_EXCEPTION_INFORMATION = *mut MINIDUMP_EXCEPTION_INFORMATION;

ENUM! {enum MINIDUMP_STREAM_TYPE {
    UnusedStream                = 0,
    ReservedStream0             = 1,
    ReservedStream1             = 2,
    ThreadListStream            = 3,
    ModuleListStream            = 4,
    MemoryListStream            = 5,
    ExceptionStream             = 6,
    SystemInfoStream            = 7,
    ThreadExListStream          = 8,
    Memory64ListStream          = 9,
    CommentStreamA              = 10,
    CommentStreamW              = 11,
    HandleDataStream            = 12,
    FunctionTableStream         = 13,
    UnloadedModuleListStream    = 14,
    MiscInfoStream              = 15,
    MemoryInfoListStream        = 16,
    ThreadInfoListStream        = 17,
    HandleOperationListStream   = 18,
    TokenStream                 = 19,
    JavaScriptDataStream        = 20,
    SystemMemoryInfoStream      = 21,
    ProcessVmCountersStream     = 22,
    IptTraceStream              = 23,
    ThreadNamesStream           = 24,

    ceStreamNull                = 0x8000,
    ceStreamSystemInfo          = 0x8001,
    ceStreamException           = 0x8002,
    ceStreamModuleList          = 0x8003,
    ceStreamProcessList         = 0x8004,
    ceStreamThreadList          = 0x8005, 
    ceStreamThreadContextList   = 0x8006,
    ceStreamThreadCallStackList = 0x8007,
    ceStreamMemoryVirtualList   = 0x8008,
    ceStreamMemoryPhysicalList  = 0x8009,
    ceStreamBucketParameters    = 0x800A,     
    ceStreamProcessModuleMap    = 0x800B,
    ceStreamDiagnosisList       = 0x800C,

    LastReservedStream          = 0xffff,
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
    CallbackOutput: PMINIDUMP_CALLBACK_OUTPUT,
);

STRUCT! {struct MINIDUMP_CALLBACK_INFORMATION {
    CallbackRoutine: MINIDUMP_CALLBACK_ROUTINE,
    CallbackParam: PVOID,
}}
pub type PMINIDUMP_CALLBACK_INFORMATION = *mut MINIDUMP_CALLBACK_INFORMATION;

pub type RVA = DWORD;
pub type RVA64 = ULONG64;

STRUCT! {struct MINIDUMP_LOCATION_DESCRIPTOR {
    DataSize: ULONG32,
    Rva: RVA,
}}
pub type PMINIDUMP_LOCATION_DESCRIPTOR = *mut MINIDUMP_LOCATION_DESCRIPTOR;

STRUCT! {struct MINIDUMP_DIRECTORY {
    StreamType: ULONG32,
    Location: MINIDUMP_LOCATION_DESCRIPTOR,
}}
pub type PMINIDUMP_DIRECTORY = *mut MINIDUMP_DIRECTORY;

extern "system" {
    pub fn MiniDumpWriteDump(hProcess: HANDLE,
        ProcessId: DWORD,
        hFile: HANDLE,
        DumpType: MINIDUMP_TYPE,
        ExceptionParam: PMINIDUMP_EXCEPTION_INFORMATION,
        UserStreamParam: PMINIDUMP_USER_STREAM_INFORMATION,
        CallbackParam: PMINIDUMP_CALLBACK_INFORMATION
    ) -> BOOL;

    pub fn MiniDumpReadDumpStream(
        BaseOfDump: PVOID,
        StreamNumber: ULONG,
        Dir: PMINIDUMP_DIRECTORY,
        StreamPointer: *mut PVOID,
        StreamSize: *mut ULONG,
    ) -> BOOL;
}
