Engine :
 engine with textures , canvas ,the works!

Camera:
  camera module
      -In production not completed yet to provide valid docs

World consists of a world with the following functions:
  location:src/world/

    - World.load_static_entities(file:&str)
      - SYNOPSIS:Loads static entities from a file to world file is the path to file
    - World.get_adj_indices(ent:&dynamicEntity::Entity)
      - SYNOPSIS: Gets the blocks adjacent to the block the entity is in:

        //  [x] | [x][x] | [ ]
        //  ----|--------|------
        //  [x] | [0][x] | [ ]
        //  [x] | [x][x] | [ ]
        //  ----|--------|------
        // When the gameobject is in the position marked [0],
        //  The blocks [0] and [x] are returned (just the indices)
        // where each |------|
        //            | [][] |
        //            | [][] |
        //            |------|
        // is a block

    - World.get_adj_objs(&self,ent:&dynamicEntity::Entity)->Vec<&staticEntity::StaticEntity>
        - SYNOPSIS: Returns a vec of entities (immutable pointers) to check collisions , render and stuff
                    from the blocks in the  World.get_adj_indices() [see the function before this]


  Loader consists of the following:
    src/asset_loader/mod.rs

    -load_world_static_entities(filename:&str)->Vec<StaticEntity>
      -SYNOPSIS:Loads static entities from the file name , the format is in assets/examples/static_entities.json
    load_dynamic_entities(filename:&str)->Vec<dynamicEntity::Entity>
      -SYNOPSIS : almost same as static entity , but for dynamic entities , the format is in assets/examples/dynamic_entities.json
