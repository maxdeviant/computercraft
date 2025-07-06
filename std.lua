local std = {}

--- Returns whether the current script is running as the main script.
---
--- @return boolean
function std.is_main()
    -- We're adding an additional frame to account for the `std` module being loaded.
    return not pcall(debug.getlocal, 4 + 1, 1)
end

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
