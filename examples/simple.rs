#[macro_use]
extern crate microstate;

microstate!{
    MicroMachine { New }
    states { New, Confirmed, Ignored }

    confirm {
        New => Confirmed
    }

    ignore {
        New => Ignored
    }

    reset {
        Confirmed => New
        Ignored   => New
    }
}

fn main() {
    let mut machine = MicroMachine::new();

    println!("{:?}", machine.confirm());
    println!("{:?}", machine.state());

    println!("{:?}", machine.ignore());
    println!("{:?}", machine.state());

    println!("{:?}", machine.reset());
    println!("{:?}", machine.state());

    println!("{:?}", machine.ignore());
    println!("{:?}", machine.state());
}
