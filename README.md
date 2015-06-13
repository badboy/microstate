MicroState
=========

Minimal Finite State Machine.

Description
-----------

A finite state machine is in only one state at a time.
From there it can change from one state to another when initiated by an triggering event: the transition.
A finite state machine is fully defined by a list of states and the transitions triggering a change from one state to another.

And that's all this crate does: it let's you define the states and transitions.
The rest is up to you.

Inspired by [@soveran](https://twitter.com/soveran)'s [micromachine](https://github.com/soveran/micromachine) in Ruby.

Documentation
-------------

[Online documentation](http://badboy.github.io/microstate/microstate)

Usage
-----

First you need to import the macro:

```rust
#[macro_use] extern crate microstate;
```

You can then create a new state machine and call transitions.

```rust
microstate!{
    MicroMachine { New };
    states { New, Confirmed, Ignored };

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

let mut machine = MicroMachine::new();

machine.confirm(); // => Some(Confirmed)
machine.state();   // => Confirmed

machine.ignore();  // => None
machine.state();   // => Confirmed

machine.reset();   // => Some(New)
machine.state();   // => New

machine.ignore();  // => Some(Ignored)
machine.state();   // => Ignored
```

## Contribute

If you find bugs or want to help otherwise, please [open an issue](https://github.com/badboy/microstate/issues).

## License

MIT. See [LICENSE](LICENSE).
