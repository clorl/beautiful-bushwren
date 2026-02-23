local overseer = require("overseer")

overseer.register_template({
  name = "bevy example",
  params = {
    example_name = {
      type = "string",
      desc = "Example name (e.g. 3d_scene)",
    },
    bevy_path = {
      type = "string",
      desc = "Path to Bevy clone",
      default = vim.fn.expand("~/Projets/bevy/bevy"), -- Update this path
    },
  },
  builder = function(params)
    return {
      cmd = { "cargo", "run", "--example", params.example_name, "--features", "bevy/dynamic_linking" },
      cwd = params.bevy_path,
      components = {
        "default",
        "unique",
        "on_complete_dispose"
      },
    }
  end
})

require("dap").configurations.rust = {
  {
    name = "Debug Nickel2Rust",
    type = "codelldb",
    request = "launch",
    program = function()
      return vim.fn.getcwd() .. '/target/debug/nickel2rust'
    end,
    cwd = '${workspaceFolder}',
    stopOnEntry = false,
  },
}
