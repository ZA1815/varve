pub mod abstraction;
pub mod api;
pub mod dependency;
pub mod modularization;
pub mod naming;
pub mod resilience;
pub mod testing;

use crate::descriptors::instruments::software::{abstraction::AbstractionInstrument, api::ApiInstrument, dependency::DependencyInstrument, modularization::ModularizationInstrument, naming::NamingInstrument, resilience::ResilienceInstrument, testing::TestingInstrument};

pub enum SoftwareInstrument {
    Abstraction(AbstractionInstrument),
    Api(ApiInstrument),
    Dependency(DependencyInstrument),
    Modularization(ModularizationInstrument),
    Naming(NamingInstrument),
    Resilience(ResilienceInstrument),
    Testing(TestingInstrument)
}