
function big_ol_function(entity)
    entity.x=32+math.cos(os.clock()*4.)*32.
    entity.y=32+math.sin(os.clock()*4.)*32.
    return entity --make_ent(p.x+1.,p:get_y())
end

return big_ol_function

