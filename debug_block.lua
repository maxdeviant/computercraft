local function main()
    local has_block, data = turtle.inspect()
    if has_block and type(data) == "table" then
        local file = io.open("debug.txt", "w")
        io.output(file)
        io.write(textutils.serialize(data))
        io.close(file)
    end
end

main()
