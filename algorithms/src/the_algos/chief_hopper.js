// https://www.hackerrank.com/challenges/chief-hopper/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign

function chiefHopper(arr)
{
    let botEnergy = 0;
    if (arr.length > 0) {
        let currentEnergy = botEnergy;
        let restart = true;

        while (restart) {
            for (let i = 0; i <= arr.length; i++) {
                let newEnergy = calculateNewEnergy(arr[i], currentEnergy);

                if (newEnergy < 0) {
                    botEnergy += 1;
                    currentEnergy = botEnergy;
                    break;
                }

                currentEnergy = newEnergy;
                if (i + 1 === arr.length) {
                    restart = false;
                }
            }
        }
    }
    return botEnergy;
}

function calculateNewEnergy(height, energy) {
    if (height > energy) {
        return energy - (height - energy);
    }
    return energy + (energy - height);
}
