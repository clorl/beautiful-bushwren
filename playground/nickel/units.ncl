let Cost = {
  # The 'item' function: item "Gold" 50
  item = fn name quantity => 
    # This check happens at COMPILE TIME
    if std.record.has_field name Items then
      { 
        type = 'Single, 
        item_id = Items."${name}".id, 
        amount = quantity 
      }
    else 
      std.fail_with "Unknown item: ${name}",

  # Logic Composers
  and = fn c1 c2 => { type = 'AllOf, requirements = [c1, c2] },
  or = fn c1 c2 => { type = 'AnyOf, requirements = [c1, c2] },
} in
{
  fireball = {
    name = "Fireball",
    
    # --- TIME ---
    cast_time = seconds 2,
    duration = rounds 1,
    cooldown = minutes 1.5,

    # --- DISTANCE ---
    range = feet 150,
    explosion_radius = meters 6,
    knockback_distance = tiles 2,

    # --- ANGLES ---
    spread_angle = deg 35,
    rotation_per_tick = rad 0.01,

    # --- PROBABILITY & RATIO ---
    crit_chance = percent 15,
    proc_legal_check = chance_1in 20,
    damage_falloff = ratio 0.5,

    # --- MASS & PHYSICS ---
    projectile_weight = lbs 2,
    impact_force = kg 10,

    # --- ECONOMY ---
    casting_cost = item "Gold" 50
    component_worth = silver 15,
		craft_recipe = and (item "BatGuano" 1) (item "Silver" 5),
		tool_requirement = or (item "Pickaxe" 1) (item "Drill" 1),

    # --- COSMETICS & LIGHT ---
    glow_color = hex "FF4500",
    light_intensity = lux 1000,
    color_temp = kelvin 4500,
  },

  ranger_dash = {
    name = "Fleet Foot",
    
    # Combining units for complex derived values
    speed_boost = percent 25,
    distance = feet 30,
    stamina_drain = per_second 5,
  }
}
