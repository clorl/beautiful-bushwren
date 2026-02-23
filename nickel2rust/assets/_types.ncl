let Type = { _rs_type | String | optional, 
	_rs_derive | Array String | default = [],
	.. 
} in {
  Component | Type = { _rs_derive = ["bevy::prelude::Component"] },
	Transform | Component = { _rs_type = "bevy::prelude::Transform" },
}
