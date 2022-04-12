local function dump(o)
    if type(o) == 'table' then
        local s = '{ '
        for k,v in pairs(o) do
            if type(k) ~= 'number' then k = '"'..k..'"' end
            s = s .. '['..k..'] = ' .. dump(v) .. ','
        end
        return s .. '} '
    else
        return tostring(o)
    end
end


function print(...)
    local coll = {...}
    local str = ""
    for k,v in ipairs(coll) do
        str = str.." "..dump(v)
    end
    __PRINT(str)
end

function __CREATE_MOCK_FROM_TBL()
    for k,v in ipairs(__FUNCS) do
        _G[v] = function()
            return function() end --just in case
        end
    end
end

function intoTbl(val)
    if type(val) == "table" then
        return val
    end
    return {val}
end

function __CREATE_EXTRACTOR_FROM_TBL()
    for k,v in ipairs(__COLLECTORS) do
        _G[v] = function(data)
            __PUSH_TBL(intoTbl(data))
        end
    end
end
