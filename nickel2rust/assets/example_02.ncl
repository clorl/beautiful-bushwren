let T = import "_types.ncl" in
{
  Health | T.Component = {
    brute | Number | default = 0,
    burn | Number | default = 0,
    toxin | Number | default = 0,
    oxy | Number | default = 0,
    max | Number | default = 100,
  },

  Tool | T.Component = {
    tool_type | [| 'Screwdriver, 'Wrench, 'Wirecutters, 'Welder |] = 'Screwdriver,
    is_active = false,
  },

  GasContainer | T.Component = {
    moles = { o2 = 0, n2 = 0, plasma = 0, co2 = 0 },
    temperature = 293.15, # Kelvins, obviously
    volume = 70.0,
  },

  # For mobs (Players/NPCs)
  Mind | T.Component = {
    key = "Guest-123",
    role = "Assistant",
    is_ghost = false,
  },

	Name | T.Component = {
		value | String = ""
	}
}
