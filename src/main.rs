use clap::Parser;
use gen::Nature;
use std::{thread, time};

mod gen;
mod space;

#[derive(Parser)]
#[clap(about, long_about = None)]
struct Cli {
    /// Custom basic configuration size
    #[clap(short, long, value_name = "NUMBER")]
    custom: Option<usize>,

    /// Random basic configuration size
    #[clap(short, long, value_parser, value_name = "NUMBER")]
    random: Option<usize>,

    /// Generation speed in milliseconds
    #[clap(long, value_parser, value_name = "MILLISECONDS")]
    speed: Option<u64>,
}

fn render(space: &space::Space) {
    print!("{esc}c", esc = 27 as char); // clean previous output

    let dim = space.dim();

    println!("\r");
    for r in 0..dim {
        print!("|");
        for c in 0..dim {
            let t = space.field.get(r).unwrap().get(c).unwrap();
            if *t == space::State::Alive {
                print!("▇");
            } else {
                print!(" ");
            }

            print!("|");
        }
        println!();
    }
}

fn main() {
    let args = Cli::parse();

    let speed_milliseconds = args.speed.unwrap_or_default();
    let five_hundred_millisecond = time::Duration::from_millis(if speed_milliseconds == 0 {
        500
    } else {
        speed_milliseconds
    });

    let mut dim = 0;
    let nature = if args.random.is_some() {
        dim = args.random.unwrap();
        Nature::Random
    } else if args.custom.is_some() {
        dim = args.custom.unwrap();
        Nature::Custom
    } else {
        Nature::Default
    };

    if dim == 0 {
        println!("Ha-ha, but no.");
        return;
    }

    let mut space = gen::gen_space(nature, dim);

    let mut i = 1;
    loop {
        i += 1;

        render(&space);
        space = space::evaluate(&space);
        println!("{} iter", i);

        thread::sleep(five_hundred_millisecond);
    }
}
