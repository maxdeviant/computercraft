---@meta

---A pure Lua implementation of the built-in
---[require](https://www.lua.org/manual/5.1/manual.html#pdf-require) function and
---[package](https://www.lua.org/manual/5.1/manual.html#5.3) library. This is
---usually not needed as it is already injected into every program environment,
---but it may be useful when building a custom shell or when executing programs
---yourself.
------
---[Official Documentation](https://tweaked.cc/library/cc.require.html)
local M = {}

---Create a new `require` function to load packages in a new `package` library.
---Can be used when creating a custom shell or when running programs outside the
---shell.
---@param env table Environment to load packages into
---@param dir string Directory that relative packages are loaded from
---@return fun(moduleName: string): unknown, unknown
---@return packagelib
------
---[Official Documentation](https://tweaked.cc/library/cc.require.html#v:make)
function M.make(env, dir) end

return M
