


# Reference

## File

### File.close
*Close this file.*
- **Signature:** `() -> ()`

### File.open
*Open the file at the provided path.*
- **Signature:** `Path -> Result<File, FileError>`

### File.path
*This file's path.*
- **Signature:** `() -> Path`

### File.read
*Read the contents of the file at the provided path into a vector of bytes.*
- **Signature:** `Path -> Result<Vec<u8>, FileError>`

### File.rename
*Move the file from the first path to the second, replacing the file at the second if it exists.*
- **Signature:** `(Path, Path) -> Result<(), FileError>`

### File.write
*Write the provided bytes to the file located at this file's path.*
- **Signature:** `[u8] -> Result<(), FileError>`



---

## SharedMemory

### SharedMemory.alloc
*Allocate a new shared memory buffer.*
- **Signature:** `(Path, usize) -> Result<SharedMemory, SharedMemoryError>`

### SharedMemory.open
*Open an existing shared memory buffer.*
- **Signature:** `Path -> Result<SharedMemory, SharedMemoryError>`



---

## Window

### Window.monitor
*The monitor on which this window resides.*
- **Signature:** `() -> Option<Monitor>`

### Window.name
*This window's name.*
- **Signature:** `() -> Option<String>`

### Window.notify
*Send a notification to the windowing system for this window.*

### Window.request
*Send a request to the windowing system for this window, and wait on its reply.*

### Window.scale
*This window's display scaling.*
- **Signature:** `() -> f64`



---

## Display

### Display.close
*Close this display connection.*
- **Signature:** `() -> Option<Monitor>`

### Display.main_monitor
*The user's primary monitor, if there is one.*
- **Signature:** `() -> ()`

### Display.open
*Open a connection to the user's display.*
- **Signature:** `() -> Result<Display, DisplayError>`



---

## Monitor

### Monitor.name
*This monitor's name.*
- **Signature:** `() -> Option<String>`

### Monitor.size
*This monitor's size, in pixels.*
- **Signature:** `() -> Vec2`

### Monitor.position
*This monitor's position, relative to the origion for the user's display setup, in pixels.*
- **Signature:** `() -> Vec2`



---

## UserInterface



---

## EventHandler
