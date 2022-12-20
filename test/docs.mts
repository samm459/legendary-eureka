// global objects available by the platinum runtime
// ------------------------------------------------
// System
// Process
// File
// Network
// Module

// System
// ------------------------------------------------
// System.platform -- The system's platform, e.g. "linux"
// System.architecture -- The system's architecture, e.g. "x86_64"
// System.name -- The system's name, e.g. 'codespaces-b55cb3'

// Process
// ------------------------------------------------
// Process.id -- The pid of the main process
// Process.stdinSync -- An syncronous, blocking iterator over the stdin of the process

// File
// ------------------------------------------------
// File.readSync(filename) -- Syncronously read a file

// Network
// -------------------------------------------------
// Network.bindSync(port) -- Syncronously bind to a port on the machine, returns a SyncServer, which is an iterator