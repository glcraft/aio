use crate::generators::llama::stop::{stop_manager, StopManager};

#[test]
fn stops() {
    let mut manager = stop_manager!("Bonjour", "bonjour", "salut");
    assert!(manager.check("Bonjour"));
    assert!(manager.check("bonjour"));
    assert!(manager.check("salut"));
    assert!(!manager.check("Au revoir"));
    manager.reset();
    assert!(!manager.check("aloa aloa"));
    manager.reset();
    assert!(!manager.check("aloa"));
    assert!(manager.check("bonjour"));
    manager.reset();
    assert!(!manager.check("aloa bon"));
    assert!(manager.check("jour"));
    manager.reset();
    assert!(!manager.check("aloa bon"));
    assert!(!manager.check("jo"));
    assert!(manager.check("ur"));
    manager.reset();
    for c in "aloa au revoir salu".chars() {
        assert!(!manager.check(&c.to_string()));
    }
    assert!(manager.check(&'t'.to_string()));
}