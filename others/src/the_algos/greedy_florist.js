function getMinimumCost(friends, flowers) {
    let cost = 0;
    let previous_cost = 0;
    let total = 0;
    flowers.sort((a, b) => b-a);
    while(total < flowers.length){
        for (let i = 0; i < flowers.length; i++) {
            cost += flowers[i] * (previous_cost+1);
            total+=1;

            if ((i + 1) % friends === 0) {
                previous_cost += 1;
            }
        }
        
    }
    
    return cost;
    
}