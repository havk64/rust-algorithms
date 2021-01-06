use {
    rand::{
        rngs::ThreadRng,
        Rng,
        distributions::{
            Alphanumeric,
            Uniform,
            Standard,
        }
    },
    rand_distr::{
        Distribution,
        Normal, 
        NormalError
    },
};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn uniform_distribution(rng: &mut ThreadRng) {
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn normal_distribution(rng: &mut ThreadRng) -> Result<(), NormalError> {
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(rng);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}

fn random_alphanumeric_password(rng: &mut ThreadRng) {
    let alphanumeric_password: String = rng
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("Alphanumeric password: {}", alphanumeric_password);
}

fn main() -> Result<(), NormalError>{
    let mut rng: ThreadRng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
    // Generate values for custom types:
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple {:?}", rand_tuple);
    println!("Randon Point: {:?}", rand_point);
    // Uniform Distribution example:
    uniform_distribution(&mut rng);
    // Normal Distribution example:
    normal_distribution(&mut rng)?;
    random_alphanumeric_password(&mut rng);
    Ok(())
}