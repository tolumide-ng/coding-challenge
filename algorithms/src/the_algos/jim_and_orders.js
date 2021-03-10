function jimOrders(orders) {
    const arrDic = [];

    console.log("the arr is", orders);
    for (let i = 0; i < orders.length; i++) {
        arrDic.push({
            customer: i + 1,
            serverTime: orders[i][0] + orders[i][1],
        });
    }

    arrDic.sort((a, b) => a.customer - b.customer);
    arrDic.sort((a, b) => a.serverTime - b.serverTime);
    const retArr = arrDic.map((cont) => cont.customer);
    return retArr;
}
