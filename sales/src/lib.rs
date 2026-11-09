#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item(pub &'static str, pub f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Store<const N: usize> {
    pub products: [Item; N],
}

impl<const N: usize> Store<N> {
    pub const fn new(products: [Item; N]) -> Self {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<Item>,
}

impl Cart {
    pub const fn new() -> Self {
        Cart { items: Vec::new() }
    }

    pub fn insert_item_by_name<const N: usize>(
        &mut self,
        s: &Store<N>,
        name: &str,
    ) -> Result<(), ()> {
        let item = s.products.iter().find(|i| i.0 == name).ok_or(())?;
        self.items.push(*item);
        Ok(())
    }

    pub fn generate_receipt(&self) -> Vec<f64> {
        let mut prices: Vec<f64> = self.items.iter().map(|i| i.1).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total: f64 = prices.iter().sum();
        if total == 0.0 {
            return prices;
        }

        // One free item per three: the cheapest ones.
        let free: f64 = prices.iter().take(prices.len() / 3).sum();
        let ratio = (total - free) / total;

        prices
            .iter()
            .map(|p| (p * ratio * 100.0).round() / 100.0)
            .collect()
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}