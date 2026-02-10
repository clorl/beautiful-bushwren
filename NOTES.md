# Custom engine structure

- Using rhai for data definition and having code as data
- Prototype chain type shit
- Strong native typing
- Hot reloadable in dev mode and baked in release
- Data becomes a Resource from evaluating the rhai files

```rhai
// skills/explosion.rhai
extends("skills/base.rhai");
#{
    id: "Explosion",
    name: "Big Boom", // Auto replace with loc string if exists
    params: {
        range: u32 = 10;
    },
    script |ctx| {
        // ctx is destructured in the current scope so using foo() is the same as ctx.foo()

        world
        .query()
        .with("Transform", "Unit")
        .filter(|e| {       // e has a reference to the components chosen in the above query
            return e.unit.isEnemy && e.transform.distance(skill.entity.transform) <= skill.params.range;
        })
        .run(|e| {      // Run for each entity found in the query
            commands.damage(e, 10);     // The damage system will do the damage if the entity has the proper components or ignore otherwise
        })
    }
}

// In the script the ctx type is a DefaultCtx, a Rust-defined struct. It could be specified to fit multiple run contexts (battle, world, skill etc)
// skills/base.rhai
#{
    id: "BaseSkill",
    name: "Base Skill",
    params: {},
    // context example values
    // context: BattleCtx // This is a rust type
    // context: MergeCtx<DefaultCtx, BattleCtx, SkillCtx>
    script |ctx| {
        // ctx is just a Rust struct with immutable pointers to game data, commands etc
        // Example defaultCtx:
        // world : The ECS World type
        // data : The current "data unit" object
        // entity :
        // references to schedules or events like EndRound, SkillExec etc
    }
}

// TODO: How do these data definitions relate to components?
// Prototype definition that then instantiate a Component? maybe
```
