enum Resources {
    Food,
    Wood,
    Tools,
    Ore,
    Metal,
}

struct ResourceEntry {
    resource: Resources,
    amount: i32,
}

impl ResourceEntry {
    fn new(resource: Resources, amount: i32) -> ResourceEntry {
        ResourceEntry {
            resource: resource,
            amount: amount,
        }
    }
}

trait Agent {
    fn new<'a>(&'a mut Vec<ResourceEntry>) -> Self;
}

struct Miner<'a> {
    inventory: &'a mut Vec<ResourceEntry>,
}

impl<'a> Agent for Miner<'a> {
    fn new(starting_resource: &'a mut Vec<ResourceEntry>) -> Miner {
        Miner { inventory: starting_resource }
    }
}

fn main() {
    let mut resource = ResourceEntry::new(Resources::Food, 3);
    let mut vec = vec![resource];
    let miner: Miner = Miner::new(vec);

    miner.perform();
}
