use super::*;

#[derive(Default)]
pub struct SurfaceBuilder {
    primary_behaviour: Option<RayBehaviour>,
    rat: Option<(f64, f64, f64)>,
    dielectric_properties: Option<DielectricProperties>,
}

#[derive(Debug)]
pub enum SurfaceBuilderError {
    RATSum,
    Unspecified,
}

fn infer_primary_behaviour(r: f64, a: f64, t: f64) -> RayBehaviour {
    use RayBehaviour::*;
    if r > a && r > t {
        Reflect
    } else if a > r && a > t {
        Absorb
    } else if t > r && t > a {
        Transmit
    } else {
        panic!("Unable to infer primary ray behaviour, because two or more of reflectance, absorption and transmittance were equal.")
    }
}

impl SurfaceBuilder {

    pub fn thin_lens(mut self, focal_length: f64) -> Self {
        self.primary_behaviour = Some(RayBehaviour::Transmit);
        self.rat = Some((0.0, 0.0, 1.0));
        self.dielectric_properties = Some(DielectricProperties::ThinLens(focal_length));
        self
    }

    pub fn with_primary_behaviour(mut self, primary_behaviour: RayBehaviour) {
        self.primary_behaviour = Some(primary_behaviour);
    }

    pub fn with_rat(mut self, reflectance: f64, absorption: f64, transmittance: f64) {
        self.rat = Some((reflectance, absorption, transmittance));
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) {
        self.dielectric_properties = Some(DielectricProperties::Constant(refractive_index));
    }

    fn infer_primary_behaviour(&self) -> Result<RayBehaviour, SurfaceBuilderError> {
        let (r, a, t) = if let Some(rat) = self.rat {
            rat
        } else {
            return Err(SurfaceBuilderError::Unspecified)
        };
        Ok(infer_primary_behaviour(r, a, t))
    }

    fn rat_sum(&self) -> Option<f64> {
        let (r, a, t) = self.rat?;
        Some(r + a + t)
    }

    fn infer_rat(&self) -> Result<(f64, f64, f64), SurfaceBuilderError> {
        use RayBehaviour::*;
        match self.primary_behaviour.as_ref().ok_or(SurfaceBuilderError::Unspecified)? {
            Reflect => Ok((1.0, 0.0, 0.0)),
            Absorb => Ok((0.0, 1.0, 0.0)),
            Transmit => Ok((0.0, 0.0, 1.0)),
        }
    }

    pub fn build(self) -> Result<SurfaceProperties, SurfaceBuilderError> {
        // use specified behaviour, otherwise infer and raise error if cannot infer
        let primary_behaviour = if let Some(primary_behaviour) = self.primary_behaviour {
            primary_behaviour
        } else {
            self.infer_primary_behaviour()?
        };
        // check rat has been set, if so check it equals 1, otherwise infer from primary behaviour
        let (reflectance, absorption, transmittance) = 
        if self.rat.is_some() {
            let rat_sum: f64 = self.rat_sum().ok_or(SurfaceBuilderError::RATSum)?;
            if ulps_eq!(rat_sum, 1.0) {
                self.rat.unwrap()
            } else {
                return Err(SurfaceBuilderError::RATSum)
            }  
        } else {
            self.infer_rat()?
        };
        // check if dielectric properties have been set, if so check they are valid, otherwise default to constant dielectric with n=1
        let dielectric_properties = if let Some(dielectric_properties) = self.dielectric_properties {
            match dielectric_properties {
                DielectricProperties::Constant(n) => if n < 1.0 { return Err(SurfaceBuilderError::Unspecified) },
                DielectricProperties::ThinLens(f) => if ulps_eq!(f, 0.0) { return Err(SurfaceBuilderError::Unspecified) },
            };
            dielectric_properties
        } else {
            DielectricProperties::Constant(1.0)
        };
        // return new surface
        Ok(SurfaceProperties {
            primary_behaviour,
            reflectance,
            absorption,
            transmittance,
            dielectric_properties,
        })
    }
}