mod dovecote;
mod simulation;

fn main() {
    let sim_range = (1000..=100000usize).step_by(1000);
    let mut results = Vec::new();

    for i in sim_range.clone()  {
        let mut sim = simulation::InitCond::new(i);
        results.push(sim.run_multiple(50));
    }

    assert_eq!(results.len(), sim_range.count());
    for result in results.iter() {
        println!("{:?}", result.0)
    }
}
