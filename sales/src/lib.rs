#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Self {
        Self { products }
    }
}

//________________________________________________________________
//

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((name, price)) = s.products.iter().find(|(n, _)| *n == ele) {
            self.items.push((name.to_string(), *price))
        };
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut receipt: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();

        receipt.sort_by(|p1, p2| p1.partial_cmp(p2).unwrap());

        let total: f32 = receipt.iter().sum();

        let discount: f32 = receipt.iter().take(receipt.len() / 3).sum();

        let percentage = discount / total;

        self.receipt = receipt
            .iter()
            .map(|&p| (((p - (p * percentage)) * 100.0).round()) / 100.0)
            .collect();

        self.receipt.clone()
    }
}
