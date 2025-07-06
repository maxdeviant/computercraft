local std = require("std")

local move = require("lib.move")

local function main(...)
    turtle.select(1)

    while true do
        local has_block, data = turtle.inspect()
        if has_block and data.name == "minecraft:cobblestone" then
            turtle.dig()
        end

        if turtle.getItemCount() == 64 then
            move.turn_around()
            turtle.drop()
            move.turn_around()
        end
    end
end

if std.is_main() then
    main(...)
end
