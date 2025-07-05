---@meta

---@alias ccTweaked.fs.openMode
---| '"r"' # read mode
---| '"w"' # write mode
---| '"a"' # append mode
---| '"r+"' # read/write, data preserved
---| '"w+"' # write, data erased
---| '"rb"' # binary read mode
---| '"wb"' # binary write mode
---| '"ab"' # binary append mode

---@alias ccTweaked.fs.seekWhence
---| '"set"' # relative to the start of the file
---| '"cur"' # relative to the current position
---| '"end"' # relative to the end of the file

---@class ccTweaked.fs.fileAttributes
---@field size integer
---@field isDir boolean
---@field isReadOnly boolean
---@field created ccTweaked.epoch
---@field modified ccTweaked.epoch
