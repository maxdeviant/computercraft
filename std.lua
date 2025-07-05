local std = {}

--- Repeats the given function *n* times.
---
--- @param n number
--- @param f fun(i: number): nil
function std.times(n, f)
    for i = 1, n do
        f(i)
    end
end

return std
