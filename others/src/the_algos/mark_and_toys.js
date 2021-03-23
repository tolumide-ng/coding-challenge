function maximumToys(prices, k) {
    prices.sort((a, b) => a - b);
    let pocket = k;
    let total = 0;

    for (let i = 0; i < prices.length; i++) {
        if (pocket - prices[i] >= 0) {
            total += 1;
            pocket -= prices[i];
        }
    }

    return total;
}
