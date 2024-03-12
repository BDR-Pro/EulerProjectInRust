# Project Euler in Rust: A Gen Z Guide 🚀

Hey fam! Welcome to the sickest coding journey where we're about to dive deep into the world of Project Euler, but with a twist – we're doing it in Rust! 🦀

## What's Project Euler? 🤔

Imagine a collection of brain-teasing math problems that are like the boss level in video games but for coding. That's Project Euler. It's where math meets coding in a battle royale style, challenging you to flex those brain muscles and coding skills simultaneously.

## Why Rust, Though? 🦀

Rust is like the superhero of programming languages. It's fast, reliable, and prevents you from making silly mistakes (goodbye, unexpected crashes!). By solving Project Euler problems in Rust, not only do you get to solve cool puzzles but also learn a language that's as sturdy as Thor's hammer but as precise as Hawkeye.

## Getting Started 🚀

### Install Rust

First things first, let's get Rusty! Install Rust by visiting [rust-lang.org](https://www.rust-lang.org/tools/install). It's like getting your digital toolbox ready.

### Clone This Repo

Git clone this repo to dive into the problems. It's like stepping into the dojo to start your training.

```bash
git clone https://github.com/BDR-Pro/EulerProjectInRust.git
cd EulerProjectInRust
```

### Battle Each Problem

Navigate to each problem like you're choosing your battles.

```bash
cargo run
```

### The Rusty Way 🛠

Rust has some cool features like ownership, borrowing, and lifetimes, making your code not just correct but also efficient. It's like learning to fight smart, not hard.

## Tips for Surviving the Euler Arena 🛡

- **Stay Calm and Rusty**: Some problems are tough. Take a break, hydrate, and come back with fresh eyes.
- **Community is Key**: Stuck? The Rust community and Project Euler forums are like your guild; they've got your back.
- **Practice Makes Perfect**: The more you code, the better you get. It's like leveling up in a game.

## Why You Should Care ✨

Solving Project Euler problems in Rust isn't just about bragging rights (though they are pretty cool). It's about sharpening your problem-solving skills, diving deep into math, and mastering a language that's built for speed and safety. It's like training to be a math wizard and coding ninja all in one.

So, what are you waiting for? Let's get Rusty and conquer Project Euler, one problem at a time! 🚀🦀

---
Remember, this journey is yours. Make it fun, make it challenging, and most importantly, make it rewarding. Let's code our way to victory! 🎉

![euler_portrait](https://github.com/BDR-Pro/project-euler/assets/91114465/0ce7a48d-2af4-4eca-92bb-4bdbc2a4a9bb)

## Explaining of my Rust code

Sure, let's break down the uses of `map`, `filter_map`, and `Some()` in the context of the Rust code provided:

### `.map()`

In Rust, `map` is a method available on iterators. What `map` does is apply a function to each item in the iterator and returns a new iterator with the results. The original iterator is not consumed or altered; instead, `map` produces a "lazy" iterator that computes the results on-the-fly as they are needed.

For example, in the code snippet:

```rust
let digits: Vec<u64> = num.to_string().chars()
                          .map(|c| c.to_digit(10).unwrap() as u64)
                          .collect();
```

Here, `num.to_string().chars()` creates an iterator over the characters of the `num`'s string representation. The `.map(|c| c.to_digit(10).unwrap() as u32)` part then applies a function to each character `c`: it converts each character to a digit (`c.to_digit(10)`), unwraps the resulting `Option<u32>` (assumes there is no error in conversion). The `map` here transforms each character in the iterator to its numeric value.

### `.filter_map()`

The `filter_map` method is also a method available on iterators. It applies a function that returns an `Option<T>` to each item in the iterator. If the function returns `Some(value)`, `filter_map` includes `value` in the new iterator it produces. If the function returns `None`, the item is excluded from the new iterator.

In the code snippet:

```rust
array.into_iter()
     .filter_map(|num| cubic_root(num))
     .collect()
```

Here, for each element `num` in `array`, `cubic_root(num)` is called. If `cubic_root(num)` returns `Some(root)`, indicating the cubic root is an integer, that `root` is included in the new iterator. If `cubic_root(num)` returns `None`, indicating there is no integer cubic root, the item is excluded. `filter_map` thus filters and maps in one step.

### `Some()`

In Rust, `Some` is a variant of the `Option` enum. The `Option` type is used when a value can be present (`Some`) or absent (`None`). This is Rust's way of handling situations that might otherwise use `null` in other languages, thus avoiding many common null-related errors.

For example, in the function:

```rust

fn cubic_root(num: u32) -> Option<u32> {
    let root = (num as f32).cbrt();
    if root.fract() == 0.0 {
        Some(root as u32)
    } else {
        None
    }
}

```

This function takes a `u32` number, computes its cubic root, and checks if the result is a whole number. If it is (`root.fract() == 0.0`), the function returns `Some(root as u32)`, indicating a successful result with the integer root wrapped in the `Some` variant. If the cubic root is not a whole number, it returns `None`, indicating the absence of an integer result.

Using `Some` and `None` instead of raw values and nulls helps Rust programs avoid many common types of runtime errors and makes functions' behavior regarding potentially absent values very clear and predictable.

### How This Code Works 🤔 **cube.rs**

1. **HashMap of Cubes**: Think of a `HashMap` like a super-advanced diary. Instead of names and phone numbers, this one matches sorted digits (like 123) with a list of cube numbers (like the Rubik's cubes but number style) that look like those digits when you shuffle them. So if you scribble down "123", your diary shows "321", "213", and so on, but only if they're legit cube numbers.

2. **The Loop**: We start checking from the cube of 345 (because we need something big enough to play with but not too huge) and shoot up from there. For each number, we cube it (like blowing up a balloon), sort its digits (getting our numbers in line, beauty pageant style), and throw that into our diary (`HashMap`). We’re keeping track of all the cube numbers that can play dress-up as each other.

3. **The Check**: Now, while we’re adding these cubes into our diary, we also peek to see if any of these dressed-up numbers have formed a cool squad of five. It's like checking if you've got five friends who all swapped clothes and still look fabulous.

4. **Eureka Moment**: Once we hit a group where five different cube numbers are basically in disguise as each other, we’ve hit jackpot. We shout out the smallest one from this group because, hey, we appreciate humility.

### Why It Is Blazing Fast 🚀

1. **No Time Wasters**: This code doesn’t mess around. It doesn’t invite every number to the party – only starts from 345 because lower numbers just can’t bring the five friends to the party (they're too small, can’t form enough permutations).

2. **Speedy Diary (HashMap)**: The `HashMap` works like flash memory. Instead of checking every possibility one by one (yawn), it groups numbers based on their digits. Think speed dating but for numbers.

3. **Bouncer at the Door (Early Exit)**: The moment we find what we want, we're out. No lingering at the bar, no waiting for the last song – we hit the solution, we grab it, we leave. Efficiency is the name of the game.

4. **Rust Under the Hood**: Rust is all about speed and memory efficiency. It's like having a Ferrari in a world of bicycles when it comes to computing stuff. Plus, Rust keeps things safe (memory-wise), so fewer crashes (bye, unexpected errors!).

5. **Streamlined Process**: Every step is calculated to avoid unnecessary work. We’re not here to dilly-dally; we’re here to find that special cube and its cool friends ASAP.
