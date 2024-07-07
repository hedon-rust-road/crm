use fake::{Dummy, Fake, Faker};
use rand::{rngs::StdRng, SeedableRng};

#[derive(Debug, Dummy)]
#[allow(unused)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
}

fn main() -> anyhow::Result<()> {
    // struct
    let f: Foo = Faker.fake();
    println!("{:?}", f);

    // tuple
    let tuple = Faker.fake::<(u8, f32, i32)>();
    println!("{:?}", tuple);

    // using `faker` module with locales
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let name: String = Name(EN).fake();
    println!("{:?}", name);
    let name: String = Name(ZH_CN).fake();
    println!("{:?}", name);

    // using convenient function without providing locale
    use fake::faker::lorem::en::*;
    let words: Vec<String> = Words(3..5).fake();
    println!("{:?}", words);

    let name_vec: Vec<String> = (Name(EN), 3..5).fake();
    println!("{:?}", name_vec);

    let name_vec = fake::vec![String as Name(EN); 3..5];
    println!("{:?}", name_vec);

    // using macro to generate nested conllection
    let name_vec = fake::vec![String as Name(EN); 4,3..5,2];
    println!("{:?}", name_vec);

    // fixed seed rng
    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let r = &mut StdRng::from_seed(seed);
    for _ in 0..5 {
        let v: usize = Faker.fake_with_rng(r);
        println!("value from fixed seed {}", v);
    }

    // Use an always true RNG so that optional types are always `Some` values. (Requires
    // always-true-rng feature).
    // use fake::utils::AlwaysTrueRng;
    // let mut rng = AlwaysTrueRng::default();
    // let result: Option<i64> = Faker.fake_with_rng(&mut rng);
    // println!("Always Some: {}", result.unwrap());

    Ok(())
}
