
/*
Abstract Factory
1. create products belonging to single family under  single interface 
*/


pub trait GetProductData {
    fn get_product(&self);
}

#[derive(Debug)]
struct BMWCAR {
    pub name: String
}

impl GetProductData for BMWCAR {
    fn get_product(&self) {
        println!("This is car {:?}", self)
    }

}

#[derive(Debug)]
struct BMWBIKE {
    pub name: String
}

impl GetProductData for BMWBIKE {
    fn get_product(&self) {
        println!("This is bike {:?}", self)
    }
}

#[derive(Debug)]
struct HONDACAR {
    pub name: String
}

impl GetProductData for HONDACAR {
    fn get_product(&self) {
        println!("This is car {:?}", self)
    }
}

#[derive(Debug)]
struct HONDABIKE {
    pub name: String
}

impl GetProductData for HONDABIKE {
    fn get_product(&self) {
        println!("This is bike {:?}", self)
    }
}

// Factory design pattern
struct ProductFactory;
enum ProductList {
    HONDABIKE,
    HONDACAR,
    BMWBIKE,
    BMWCAR
}
impl ProductFactory {
    fn build_product(product_name: ProductList) -> Box<dyn GetProductData> {
        match product_name {
            ProductList::BMWBIKE => return Box::new(BMWBIKE{
                name: String::from("BMW G310R")}),
            ProductList::BMWCAR => return Box::new(BMWCAR{
                name: String::from("BMW X1")}),
            ProductList::HONDABIKE => return Box::new(HONDABIKE{
                name: String::from("Honda Cb300R")}),
            ProductList::HONDACAR => return Box::new(HONDACAR{
                name: String::from("Honda Civic")})
        }
    }
}


/*
    Abstract Factory which gives you collection from a particular company
*/ 

struct BMW;
struct LAMBO;

enum Company {
    BMW,
    HONDA
}
struct ComboFactory;
impl ComboFactory {
    fn get_products(company_type: Company) {
        match company_type {
            Company::BMW => {
                let bmwbike = ProductFactory::build_product(ProductList::BMWBIKE);
                let bmwcar = ProductFactory::build_product(ProductList::BMWCAR);
                bmwbike.get_product();
                bmwcar.get_product();
            }
            Company::HONDA => {
                let hondabike = ProductFactory::build_product(ProductList::HONDABIKE);
                let hondacar = ProductFactory::build_product(ProductList::HONDACAR);
                hondabike.get_product();
                hondacar.get_product();
            }
        }
    }

}



fn main() {
    println!("Hello, world!");
    ComboFactory::get_products(Company::BMW);
    ComboFactory::get_products(Company::HONDA);
}
