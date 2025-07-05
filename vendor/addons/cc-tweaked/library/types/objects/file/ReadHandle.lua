---@meta

---A file handle created with `fs.open()` using `r` mode
---
------
---[Official Documentation](https://tweaked.cc/module/fs.html#ty:ReadHandle)
---@class ccTweaked.fs.ReadHandle
ReadHandle = {}

---Read a single line from the file
---@param includeNewline? boolean If any trailing newline characters should be included. Defaults to false
---@return string|nil line The read line or nil if the end of the file has been reached
---@throws If the file has been closed
------
---[Official Documentation](https://tweaked.cc/module/fs.html#ty:ReadHandle:readLine)
function ReadHandle.readLine(includeNewline) end

---Read the remainder of the file
---@return string|nil contents The remaining contents of the file or nil if at the end of the file
---@throws If the file has been closed
------
---[Official Documentation](https://tweaked.cc/module/fs.html#ty:ReadHandle:readAll)
function ReadHandle.readAll() end

---Read characters from the file
---@param count? number The number of characters to read. Defaults to 1
---@return string|nil characters The read character(s) or nil if at the end of the file
---@throws When trying to read a negative number of characters
---@throws If the file has been closed
------
---[Official Documentation](https://tweaked.cc/module/fs.html#ty:ReadHandle:read)
function ReadHandle.read(count) end

---Move the cursor to a new position in the file. Sets the cursor to a new position offset by `offset`, relative to `whence`.
---@param whence ccTweaked.io.seekMode?
---@param offset integer?
---@return integer | nil position The new cursor position or nil if failed
---@return string? err The reason that seeking failed
---@throws If the file has been closed
------
---[Official Documentation](https://tweaked.cc/module/fs.html#ty:ReadHandle:seek)
function ReadHandle.seek(whence, offset) end

---Close the file, freeing it
---
---Once closed, it can no longer be read unless it is reopened
---@throws If the file has already been closed
------
---[Official Documentation](https://tweaked.cc/module/fs.html#ty:ReadHandle:close)
function ReadHandle.close() end
