---@meta

---@alias ccTweaked.cc.shell.completion.function fun(shell: ccTweaked.shell, text: string, previousArguments: string[]): string[]

---Helper functions for use with shell completion. Most programs can have
---completion support added using `build` rather than manually creating a
---function for `shell.setCompletionFunction`.
---
---Note that the helper functions in this module do not accept an argument
---index, and thus are not directly usable with `shell.setCompletionFunction`.
---You can wrap them using `build`, or a custom function.
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html)
local M = {}

---Complete the name of a file relative to the current working directory
---@param shell ccTweaked.shell Shell to perform completion in
---@param text string Text to complete
---@return string[] suffixes Remaining text of matching files
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:file)
function M.file(shell, text) end

---Complete the name of a directory relative to the current working directory
---@param shell ccTweaked.shell Shell to perform completion in
---@param text string Text to complete
---@return string[] suffixes Remaining text of matching directories
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:dir)
function M.dir(shell, text) end

---Complete the name of a file or directory relative to the current working directory
---@param shell ccTweaked.shell Shell to perform completion in
---@param text string Text to complete
---@param previous string[] Shell arguments before this one
---@param addSpace boolean Whether to add a space after the completed item
---@return string[] suffixes Remaining text of matching directories/files
--------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:dirOrFile)
function M.dirOrFile(shell, text, previous, addSpace) end

---Complete the name of a program
---@param shell ccTweaked.shell Shell to perform completion in
---@param text string Text to complete
---@return string[] suffixes Remaining text of matching programs
-------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:program)
function M.program(shell, text) end

---Complete arguments of a program
---@param shell ccTweaked.shell Shell to perform completion in
---@param text string Text to complete
---@param previous string[] Shell arguments before this one
---@param starting integer The argument index this program and args start at
---@return string[] suffixes Remaining text of matching programs or arguments
-------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:programWithArgs)
function M.programWithArgs(shell, text, previous, starting) end

---A wrapped version of `help.completeTopic` for use with `build`
---
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:help)
function M.help(text) end

---A wrapped version of `cc.completion.choice` for use with `build`
---
-------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:choice)
function M.choice() end

---A wrapped version of `cc.completion.peripheral` for use with `build`
---
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:peripheral)
function M.peripheral() end

---A wrapped version of `cc.completion.side` for use with `build`
---
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:side)
function M.side() end

---A wrapped version of `cc.completion.setting` for use with `build`
---
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:setting)
function M.setting() end

---A wrapped version of `cc.completion.command` for use with `build`
---
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:command)
function M.command() end

---Helper function for building shell completion arguments.
---
---Accepts a series of single-argument completion functions and combines them
---into a function suitable for use with `shell.setCompletionFunction`. Every
---argument given represents an argument to the target program to be completed.
---Each argument can be:
---
---1. `nil`: will not be completed
---2. `function`: receives the `shell` object, string to complete, and array of
---arguments before this one
---3. `table`: Must have a `function`, like the above one, as the first item.
---Can also be followed by additional values that will be passed to the function
---from the start of the table. If this table is the last argument, it can set
---the `many` key to `true`, indicating that the function should be used to
---complete any remaining arguments.
---@param ... nil | ccTweaked.cc.shell.completion.function | {[1]: ccTweaked.cc.shell.completion.function, [integer]: any}
------
---[Official Documentation](https://tweaked.cc/library/cc.shell.completion.html#v:build)
function M.build(...) end

M.build({ false, true })

return M
