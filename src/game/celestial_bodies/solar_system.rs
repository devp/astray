use rand::distributions::Distribution;
use rand_distr;
use crate::game::celestial_bodies::{CelestialBody, CelestialBodyType};
use crate::game::celestial_bodies::planet::Planet;
use crate::game::celestial_bodies::star::Star;
use crate::game::helpers::{astrophysics, orbit_dynamics};

#[derive(Clone)]
pub struct SolarSystem {
    star: Star,
    planets: Vec<Planet>,
    spacing_factor: f32,
}

impl SolarSystem {
    pub fn get_n_planets(&self) -> usize {
        self.planets.len()
    }

    pub fn get_star_mass(&self) -> f32 { self.star.get_mass() }
    pub fn get_star(&self) -> Star { self.star.clone() }

    pub fn get_planets(&self) -> Vec<Planet> { self.planets.clone() }

    pub fn get_inner_limit(&self) -> f32 {
        astrophysics::calculate_system_inner_limit_from_star_radius_and_density(
            self.star.get_radius(),
            astrophysics::calculate_density_from_mass_and_radius(
                self.star.get_mass(),
                self.star.get_radius()
            )
        )
    }

    pub  fn get_nth_orbit_radius(&self, n: u32) -> f32 {
        if self.planets.len() > 0 {
            astrophysics::calculate_nth_orbit(
                self.planets[0].get_orbit_radius(),
                self.spacing_factor,
                n,
            )
        } else {
            0.0
        }
    }
}

impl CelestialBody for SolarSystem {
    type HostType = ();
    fn get_name(&self) -> String {
        self.star.get_name()
    }

    fn get_type(&self) -> CelestialBodyType {
        CelestialBodyType::SolarSystem
    }

    fn get_mass(&self) -> f32 {
        let mut r = self.star.get_mass();
        self.planets.iter().for_each(|p| {
            r += p.get_mass()
        });
        r
    }

    fn get_radius(&self) -> f32 {
        self.planets.last().unwrap().get_orbit_radius()
    }

    fn generate(host: ()) -> Self {
        let mut rng = rand::thread_rng();

        let spacing_factor = rand_distr::Normal::new(
            0.4,
            0.2
        ).unwrap().sample(&mut rng);

        let mut system = Self {
            star: Star::generate(()),
            planets: vec![],
            spacing_factor,
        };

        let n_planets: i32 = rand_distr::Normal::new(
            5.0,
            1.0
        ).unwrap().sample(&mut rng) as i32;

        for _ in 0..n_planets {
            system.planets.push(Planet::generate(system.clone()));
        }

        system
    }
}