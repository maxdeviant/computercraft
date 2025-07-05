local inventory = {}

--- Returns whether the turtle has the specified item in its inventory.
---
--- @param item string The name of the item.
function inventory.has_item(item)
    for slot = 1, 16 do
        local it = turtle.getItemDetail(slot)
        if it and it.name == item then
            return true
        end
    end

    return false
end

--- Selects the slot containing the specified item.
--- Returns whether the item was selected successfully.
---
--- @param item string The name of the item to select.
function inventory.select_item(item)
    for slot = 1, 16 do
        turtle.select(slot)
        local it = turtle.getItemDetail()
        if it and it.name == item then
            return true
        end
    end

    return false
end

return inventory
