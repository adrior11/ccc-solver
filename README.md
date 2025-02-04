# ☁️ Cloudflight Coding Contest - Rust Solver

This project is designed to streamline the process of solving levels in the [Cloudflight Coding Contest](https://register.codingcontest.org) (CCC) quickly and efficiently.
By setting up a structured approach to file handling, input/output verification, and code organization, you'll have more time to focus on developing algorithms for each challenge.

## 🛠️ Before the Contest

### Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system before running the available commands.

Clone the repository to your machine:
```bash
$ git clone git@github.com:adrior11/ccc-solver.git
$ cd ccc-solver
```

Build and run the application using cargo:
```bash
$ cargo run -- <COMMAND> [OPTIONS]
```

### Setting up the Solver

The CCC starts at level 1 and progresses as you complete each level. For each level, follow these steps:

1. Copy `level.example` to create a new level file.
2. Rename the copied file to match the level (e.g., `level1.rs`, `level2.rs`).
3. Implement the solution within the `solve` function.
4. Uncomment the relevant module in `main.rs` and the corresponding match arm in `solve_level` to include the level.

<details open>
  <summary><h4>📝 Parts to uncomment</h4></summary>

**Line 6-13**:
```rs
// NOTE: uncomment levels
mod level1;
mod level2;
mod level3;
mod level4;
mod level5;
mod level6;
mod level7;
```

**Line 124-136**:
```rs
#[allow(unused_variables)]
fn solve_level(level: usize, input_lines: Vec<String>) -> Vec<String> {
    #[allow(clippy::match_single_binding)]
    match level {
        // NOTE: uncomment levels
        // 1 => level1::solve(input_lines),
        // 2 => level2::solve(input_lines),
        // 3 => level3::solve(input_lines),
        // 4 => level4::solve(input_lines),
        // 5 => level5::solve(input_lines),
        // 6 => level6::solve(input_lines),
        // 7 => level7::solve(input_lines),
        _ => unreachable!()
    }
}
```
</details>

## ⏳ During the Contest

### Adding Level input
Each CCC level includes a zip file with specific inputs.
Move the zip file to the `src/input` directory and unzip it, creating a folder named after the level, which will contain the `.in` files as well as `example.in` and `example.out` files.
The solver uses these files for solution verification.

> [!IMPORTANT]
> Unlike the `src/output` directory, the `src/input` directory is not created automatically.
> You may need to add it manually, for example, by running `$ mkdir src/input` from the project root.

### Writting the Algorithm for a Level

The `level.example` file serves as a blueprint for each level.

- **Function Signature**: Avoid altering the function signature, as each `.in` file is read as an array of strings representing the input lines.
- **Output Vector**: The example file already provides an `out` variable of type `Vec<String>`. Use this vector to store your algorithm’s output, appending each result as a separate string in the order expected by the challenge.
- **Algorithm Implementation**: Place your core algorithm inside the provided for loop, which you may adjust to fit the specific requirements of each level.

For reference, you can see how this `solve` function is implemented in my previous level submissions, providing examples of how to structure and approach solutions across various challenges.

```rs
pub fn solve(input_lines: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    for line in input_lines.iter().skip(1) {
        for c in line.chars() {
            // NOTE: complete level algorithm
        }

        // NOTE: push output
        out.push(o);
    }

    out
}
```

<details open>
  <summary><h4>🧪 Examples of my past submissions</h4></summary>

**39th Classic CCC level1 submission**:
```rs
#[derive(Debug)]
struct Dirs {
    w: u8,
    a: u8,
    s: u8,
    d: u8,
}

impl Dirs {
    fn new() -> Self {
        Self { w: 0, a: 0, s: 0, d: 0 }
    }
}

pub fn solve(input_lines: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    for line in input_lines.iter().skip(1) {
        let mut dirs = Dirs::new();
        for c in line.chars() {
            match c {
                'W' => dirs.w+=1,
                'A' => dirs.a+=1,
                'S' => dirs.s+=1,
                'D' => dirs.d+=1,
                _ => unreachable!()
            };
        }

        let o = format!("{} {} {} {}", dirs.w, dirs.d, dirs.s, dirs.a);
        out.push(o);
    }

    out
}
```

**40th Classic CCC level1 submission**:
```rs
struct Room {
    x: usize,
    y: usize,
}

pub fn solve(input_lines: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    for line in input_lines.iter().skip(1) {
        let room_size: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let room = Room { x: room_size[0], y: room_size[1] };

        let result = room.x / 3 * room.y;

        let o = format!("{result}");
        out.push(o);
    }

    out
}
```

**40th Classic CCC level2 submission**:
```rs
const DESK_SIZE: usize = 3;

#[derive(Debug)]
struct Room {
    room: Vec<Vec<usize>>,
    width: usize,
    height: usize
}

impl Room {
    fn new(x: usize, y: usize) -> Self {
        Room {
            room: vec![vec![0; x]; y],
            width: x,
            height: y
        }
    }

    fn add_horizontal_desk(&mut self, x: usize, y: usize, id: usize) {
        for i in 0..DESK_SIZE {
            self.room[y][x + i] = id;
        }
    }

    fn can_place_horizontal_desk(&self, x: usize, y: usize) -> bool {
        for i in 0..DESK_SIZE {
            if self.room[y][x + i] != 0 {
                return false;
            }
        }
        true
    }

    fn to_string_vector(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for row in self.room.clone() {
            let row_str = row.into_iter().map(|x| x.to_string()).collect();
            out.push(row_str);
        }
        out
    }

    fn arrange_desks_in_room(&mut self) {
        let mut current_desk = 1;

        // Arrange desk horizontally
        for y in 0..self.height {
            for x in 0..self.width {
                if self.can_place_horizontal_desk(x, y) {
                    self.add_horizontal_desk(x, y, current_desk);
                    current_desk += 1;
                }
            }
        }
    }
}

pub fn solve(input_lines: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    for line in input_lines.iter().skip(1) {
        let room_args: Vec<usize> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut room = Room::new(room_args[0], room_args[1]);
    
        room.arrange_desks_in_room();

        for row in room.to_string_vector().iter() {
            out.push(row.to_string());
        }
    }

    out
}
```

</details>

> [!NOTE]
> Since the levels build on one another, you can copy your code from a previous level to the next and enhance it as you go.

### Running the solver 
The solver runs the algorithm on `example.in` and, if the output matches `example.out`, processes each `.in` file for the level, generating `.out` files.
You can also run the solver for specific sub-levels for debugging and verification.

> [!NOTE]
> To force output writing, set the `FORCE_WRITE` constant to true in `main.rs` (helpful for levels with multiple solutions).
> This does not apply to individual sub-level runs.

**List of available commands**:
```sh
$ cargo run -- -h

  Usage:
    $ cargo run -- <LEVEL_NUMBER> [SUB_LEVEL_NUMBER]

    Arguments:
      <LEVEL_NUMBER>      The level to solve
      [SUB_LEVEL_NUMBER]  The optional sub level to solve

    Examples:
      $ cargo run 1       Solve example of level 1 and, if it matches, write output files
      $ cargo run 1 3     Solve sub level 3 of level 1 without writing a output file
```

### Submit the output & level files 

Output files for each level are stored in the `src/output` directory, making submission to the CCC [CatCoder](https://catcoder.codingcontest.org) platform straightforward. 
Select the `.out` files and submit.

If correct, you may be asked to submit your level’s code, gaining 2 additional minutes for the contest. 
Since each level's algorithm is isolated in its respective file (e.g., `level1.rs`, `level2.rs`), submitting these files is also straightforward.

## 📁 File Structure

```bash
.
├── Cargo.toml
└── src           # Source code (rust root folder)
    ├── input     # Directory containing the input files for each level
    │   ├── *     # Folder for each level (Created from the .zip)
    │   └── level1
    │       ├── level1_1.in
    │       ├── level1_2.in
    │       ├── level1_3.in
    │       ├── level1_4.in
    │       ├── level1_5.in
    │       ├── level1_example.in
    │       └── level1_example.out
    ├── output    # Directory containing the files generated by the solver 
    │   ├── *     # Folder for each level
    │   └── level1
    │       ├── level1_1.out
    │       ├── level1_2.out
    │       ├── level1_3.out
    │       ├── level1_4.out
    │       └── level1_5.out
    ├── main.rs   # Main application entry
    ├── *         # Additional Rust files for each level
    ├── level1.rs # Level 1 files containing a solve function
    └── level.example # Template for new levels
```

## License

This project is open source and available under the MIT License.
