//! Integration tests for SigmaOS-arch-dependency-manager.

use sigmaos_arch_dependency_manager_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
