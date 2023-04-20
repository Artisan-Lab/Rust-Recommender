use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Agent {
    id: i64,
    state: i32,
}

struct CellState {
    state: i64,
}

struct Chart {
    agents: HashMap<usize, RefCell<Agent>>,
    grid: Vec<CellState>,
}

impl Chart {
    fn new(size: usize) -> Self {
        let mut agents = HashMap::new();
        agents.insert(1, RefCell::new(Agent { id: 1, state: 0 }));
        agents.insert(2, RefCell::new(Agent { id: 2, state: 1 }));

        let mut grid: Vec<CellState> = Vec::with_capacity(size);

        Self { agents, grid }
    }

    fn do_stuff(&self, agent: &RefCell<Agent>) {
        for other in self.agents.values().filter(|&other| agent != other) {
            if other.borrow().state == 1 {
                agent.borrow_mut().state += 1;
            }
        }
    }

    fn step_agents(&self) {
        for agent in self.agents.values() {
            self.do_stuff(agent);
        }
    }
}

fn main() {
    let mut chart = Chart::new(128);
    chart.step_agents();
    
    for agent in chart.agents.values() {
        println!("{:?}", agent);
    }
}