pub struct Asset{
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

pub struct Portifolio{
    pub assets: Vec<Asset>,
}

impl Portifolio{
    pub fn new() -> Portifolio{
        Portifolio{ assets: vec![] }
    }

    pub fn add_asset(&mut self, asset: Asset){
        self.assets.push(asset);
    }

    pub fn total_value(&self)->f64{
        self.assets.iter().map(|asset| asset.price*asset.quantity as f64).sum()
    }
}