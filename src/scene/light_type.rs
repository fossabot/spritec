use crate::math::{Rgb, Radians};

#[derive(Debug, Clone)]
pub enum LightType {
    /// Point lights emit light in all directions from their position in space. Rotation and scale
    /// are ignored other than for their affect on the position.
    ///
    /// The brightness of the light attenuates in a physically correct manner as distance increases
    /// from the light's position (i.e. brightness goes like the inverse square of the distance).
    Point {
        /// The color of the light in linear space
        color: Rgb,

        /// The intensity of the light in candela (lm/sr)
        intensity: f32,

        /// Hint defining a distance cutoff at which the light's intensity may be considered to
        /// have reached zero. Must be non-zero. If None, range is considered to be infinite.
        range: Option<f32>,
    },

    /// Directional lights are light sources that act as though they are infinitely far away and
    /// emit light in the direction of the local -z axis. Position and scale are ignored other than
    /// for their affect on the orientation of the light.
    ///
    /// The light is not attenuated, because it is at an infinite distance away.
    Directional {
        /// The color of the light in linear space
        color: Rgb,

        /// The intensity of the light in lux (lm/m^2)
        intensity: f32,
    },

    /// Spot lights emit light in a cone in the direction of the local -z axis. Scale does not
    /// affect cone shape, and is ignored except for its effect on position and orientation.
    ///
    /// The brightness attenuates in a physically correct manner as distance increases from the
    /// light's position (i.e. brightness goes like the inverse square of the distance).
    Spot {
        /// The color of the light in linear space
        color: Rgb,

        /// The intensity of the light in candela (lm/sr)
        intensity: f32,

        /// Hint defining a distance cutoff at which the light's intensity may be considered to
        /// have reached zero. Must be non-zero. If None, range is considered to be infinite.
        range: Option<f32>,

        /// Angle, in radians, from centre of spotlight where falloff begins. Must be greater than
        /// or equal to 0 and less than outer_cone_angle.
        inner_cone_angle: Radians,

        /// Angle, in radians, from centre of spotlight where falloff ends. Must be greater than
        /// inner_cone_angle and less than or equal to PI / 2.0.
        ///
        /// To disable angular attenuation, set this value to PI radians
        outer_cone_angle: Radians,
    },
}

impl<'a> From<gltf::khr_lights_punctual::Light<'a>> for LightType {
    fn from(light: gltf::khr_lights_punctual::Light<'a>) -> Self {
        let color = Rgb::from(light.color());
        // HACK: The glTF exporter for Blender has a bug where it exports the light intensity in
        //   the wrong units. This works around that issue.
        // See: https://github.com/KhronosGroup/glTF-Blender-IO/issues/564
        let intensity = light.intensity() / 1000.0;
        let range = light.range();

        use gltf::khr_lights_punctual::Kind::*;
        match light.kind() {
            Point => LightType::Point {
                color,
                intensity,
                range,
            },

            Directional => LightType::Directional {
                color,
                intensity,
            },

            Spot {inner_cone_angle, outer_cone_angle} => LightType::Spot {
                color,
                intensity,
                range,
                inner_cone_angle: Radians::from_radians(inner_cone_angle),
                outer_cone_angle: Radians::from_radians(outer_cone_angle),
            },
        }
    }
}
