# components.ncl
let T = import "_types.ncl" in
{
  Health | T.Component = {
    max | Number | default = 100,
    current | Number | default = 100,
  },
}
