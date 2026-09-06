local commands = {}

local function add(name, desc, run)
    commands[name] = { desc = desc, run = run }
end

add("sv", "Save the file", function()
    return "save", "Saved"
end)

add("sp", "Save and exit", function()
    return "save_and_exit", "Saved, exiting"
end)

add("ex", "Exit without saving", function()
    return "exit", "Exiting"
end)

add("dis", "Discard changes and exit", function()
    return "discard", "Discarded, exiting"
end)

add("clear", "Clear the buffer", function()
    return "clear", "Buffer cleared"
end)

add("f", "Find a specific word", function()
    return "Results:", "Here are the results."
end)

return commands
