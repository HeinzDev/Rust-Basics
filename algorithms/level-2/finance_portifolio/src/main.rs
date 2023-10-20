mod finance;
use finance::{Asset, Portifolio};

fn main() {
    let mut portifolio = finance::Portifolio::new();
    portifolio.add_asset(finance::Asset{
        name: "TESTE".to_string(),
        price: 144.0,
        quantity:10
    });    
    portifolio.add_asset(finance::Asset{
        name: "TESTE2".to_string(),
        price: 111.0,
        quantity: 4
    });
    println!("Valor do seu portifolio: ${}", portifolio.total_value());
}
