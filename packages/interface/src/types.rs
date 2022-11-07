use cosmwasm_schema::cw_serde;
use cosmwasm_std::Order;

#[cw_serde]
pub struct SwapRoute {
    pub pool_id: u64,
    pub token_denom: String,
}

#[cw_serde]
pub enum RangeOrder {
    Asc,
    Desc,
}

impl From<Order> for RangeOrder {
    fn from(order: Order) -> Self {
        match order {
            Order::Ascending => Self::Asc,
            Order::Descending => Self::Desc,
        }
    }
}

impl From<RangeOrder> for Order {
    fn from(order: RangeOrder) -> Self {
        match order {
            RangeOrder::Asc => Order::Ascending,
            RangeOrder::Desc => Order::Descending,
        }
    }
}
