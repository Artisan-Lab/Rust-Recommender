use std::collections::HashMap;

struct Agent { // each agent carries some state
    id: i64,
    state: i32,
}

struct CellState { // some state of a grid cell
    state: i64,
}

struct Chart {
    agents: HashMap<usize, Agent>,
    grid: Vec<CellState>,
}

impl Chart {
    fn new(size: usize) -> Chart {
        let mut agents = HashMap::new(); // generate hash and populate with 2 agents
        agents.insert(10, Agent { id: 1, state: 1 });
        agents.insert(11, Agent { id: 2, state: 0 });
        let mut grid: Vec<CellState> = Vec::with_capacity(size);

        Chart {
            agents: agents,
            grid: grid,
        }
    }

    fn do_stuff(&mut self, agent: &mut Agent) {
        // here we want to update the state of agent,
        // based on the state of other agents in the grid
    }

    fn step_agents(&mut self) {
        for (_, agent) in &mut self.agents {
            self.do_stuff(agent);
        }
    }
}

fn main() {
    let mut ch = Chart::new(128);
    ch.step_agents();
}