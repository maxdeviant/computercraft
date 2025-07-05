---@meta

---@alias ccTweaked.shell.completionFunction fun(shell: ccTweaked.shell, index: integer, argument: string, previous: string[]): string[]? The completion function to use for completion

---@alias ccTweaked.shell.completionInfo table<string, completionInfo>
---@class completionInfo
---@field fnComplete ccTweaked.shell.completionFunction
