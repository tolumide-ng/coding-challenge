#[derive(Debug)]
struct OrderDic {
    preparation_time: u8,
    order_number: u8,
}
fn jim_orders(orders: Vec<OrderDic>) -> Vec<u8> {
    let mut vec_store: Vec<OrdDetail> = vec![];

    struct OrdDetail {
        customer: u8,
        server_time: u8,
    }

    for order in 0..orders.len() {
        vec_store.push(OrdDetail {
            customer: (order + 1) as u8,
            server_time: orders[order].preparation_time + orders[order].order_number,
        });
    }

    // because sort/sort_by does not reorder already equal elements, we only sort by order_time
    vec_store.sort_by(|a, b| a.server_time.partial_cmp(&b.server_time).unwrap());

    let new_vec_store: Vec<u8> = vec_store.iter().map(|a| a.customer).collect();
    return new_vec_store;
}

#[cfg(test)]
mod test_jim_orders_cont {
    use super::*;

    #[test]
    fn test_jim_orders() {
        assert_eq!(jim_orders(vec![]), vec![]);

        assert_eq!(
            jim_orders(vec![
                OrderDic {
                    preparation_time: 3,
                    order_number: 1
                },
                OrderDic {
                    order_number: 2,
                    preparation_time: 3,
                },
                OrderDic {
                    order_number: 3,
                    preparation_time: 3,
                }
            ]),
            vec![1, 2, 3]
        )
    }
}
