MODULES = {
    cache = {},
    resolvers = {}
}

function ADD_MODULE(virtual_file_name, module)
    MODULES.resolvers[virtual_file_name] = function ()
        if MODULES.cache[virtual_file_name] then
            return MODULES.cache[virtual_file_name]
        else
            local result = module()
            MODULES.cache[virtual_file_name] = result
            return result
        end
    end
end

function REQUIRE_MODULE(virtual_file_name)
    return MODULES.resolvers[virtual_file_name]()
end