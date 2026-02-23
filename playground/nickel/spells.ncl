let Lib = {
    # Custom contract to ensure a string looks like a file path for icons
    IconPath = std.contract.from_predicate (fn path => 
        std.string.is_match "^icons/.*\.png$" path),

    # Enum for spell schools
    School = [| 'Abjuration, 'Conjuration, 'Divination, 'Enchantment, 
                'Evocation, 'Illusion, 'Necromancy, 'Transmutation |],

    # Enum for targeting logic
    TargetType = [| 'Self, 'Single, 'Area, 'Touch |],
} in

let SpellSchema = {
    name | String,
    level | Number | default = 0,
    school | Lib.School,
    
    # Visuals
    icon | Lib.IconPath | default = "icons/default_spell.png",
    
    # Requirements & Costs
    requirements = {
        vocal | Bool | default = true,
        somatic | Bool | default = true,
        material | String | optional,
        cost_gold | Number | default = 0,
    },

    # Casting Details
    casting = {
        action_type | [| 'Action, 'BonusAction, 'Reaction, 'Minute |] | default = 'Action,
        range_ft | Number | default = 60,
        concentration | Bool | default = false,
    },

    # THE COMPLEX PART: Effects as Data
    # This allows the Rust engine to parse exactly what happens on hit
    effects | {
        damage_type | [| 'Fire, 'Force, 'Necrotic, 'Radiant, 'Healing, 'None |] | default = 'None,
        dice_roll | String | doc "e.g., '1d10' or '4d8'",
        scaling_per_level | String | default = "0",
        save_type | [| 'Dexterity, 'Wisdom, 'Constitution, 'None |] | default = 'None,
        condition_applied | String | optional | doc "e.g., 'Prone' or 'Blinded'",
    },
} in

# ==========================================================
# SPELL DATABASE (Satisfying the Contract)
# ==========================================================

{
    # We apply the array contract to the whole list
    spells | Array SpellSchema = [
        # --- SCENARIO 1: Basic Cantrip ---
        {
            name = "Fire Bolt",
            level = 0,
            school = 'Evocation,
            icon = "icons/spells/fire_bolt.png",
            requirements.somatic = true,
            effects = {
                damage_type = 'Fire,
                dice_roll = "1d10",
                scaling_per_level = "1d10",
            }
        },

        # --- SCENARIO 2: Complex Level 3 Spell ---
        {
            name = "Fireball",
            level = 3,
            school = 'Evocation,
            icon = "icons/spells/fireball.png",
            requirements = {
                material = "A tiny ball of bat guano and sulfur",
            },
            casting = {
                range_ft = 150,
            },
            effects = {
                damage_type = 'Fire,
                dice_roll = "8d6",
                save_type = 'Dexterity,
                scaling_per_level = "1d6",
            }
        },

        # --- SCENARIO 3: Healing & Conditions ---
        {
            name = "Healing Word",
            level = 1,
            school = 'Evocation,
            icon = "icons/spells/healing_word.png",
            casting = {
                action_type = 'BonusAction,
            },
            requirements = {
                somatic = false, # Only Vocal
            },
            effects = {
                damage_type = 'Healing,
                dice_roll = "1d4",
                scaling_per_level = "1d4",
            }
        }
    ]
}
