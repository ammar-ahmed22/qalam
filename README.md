<div align="center">
<h1>qalam</h1>
<p>Islamic/Arabic terminology inspired, dead-simple, programming language</p>
</div>

## Table of Contents
- [Introduction](#introduction)
- [Using `qalam`](#using-qalam)
- [Syntax](#syntax)
  * [Keywords](#keywords)
  * [Conditionals](#conditionals)
  * [Loops](#loops)
  * [Objects (Classes)](#objects--classes-)
- [Native Functions](#native-functions)

## Introduction
As a learning exercise, to really hone my skills, I wanted to write a programming language myself. In order to accomplish this, I'm following the amazing tutorial called [Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/).

While Robert goes through the entire process, step-by-step, to create a programming language he calls Lox, I didn't want to simply follow a tutorial and copy-paste code. I have found that real learning occurs when you change up tutorials a little so that you can have a deeper understanding.

In order to do this, I decided to create my own syntax for the language with the same general features as Lox. I also decided to use Rust to make the language instead of Java so that I cannot possibly copy-paste code.

## Using `qalam`
1. Install the `qalam` interpreter
```bash
cargo install qalam
```

2. Create a `.qlm` file with your source code
```bash
echo "qul \"hello world!\"" > main.qlm
```

3. Run the `qalam` code
```bash
qalam main.qlm
>> hello world!
```

## Syntax
### Keywords
| Syntax | Meaning/Inspiration | Usage |
| :----: | :------------------ | :---- |
| `niyya` | Intention. Signifies the intention to store a value. | Variable declarations |
| `amal` | Good Deeds/Actions. Functions do things (actions/deeds) | Function declarations |
| `radd` | To return (Arabic) | Return statement |
| `qul` | To say (Arabic) | Print statement |
| `ghaib` | Unseen, hidden. Signifies a value is not present | Null value |

```text
niyya a = 1;
niyya b = 2;

amal add(a, b) {
  radd a + b;
}

qul add(a, b);
// prints 3
```

### Conditionals
| Syntax | Meaning/Inspiration | Usage |
| :----: | :------------------ | :---- |
| `shart` | Condition. If a condition is satisifed, do something. | If statement |
| `illa` | Else. Self-explanatory | Else statement |
| `ilshart` | Combination of `shart` and `illa` | Else-if statement |
| `haqq` | Truth. A boolean `true` is the epitome of truth. | Boolean `true` value |
| `batil` | Falsehood. A boolean `false` is the epitome of falsehood. | Boolean `false` value |
| `wa` | and (Arabic). Self-explanatory. | And operator. Can also use `&&` |
| `aw` | or (Arabic). Self-explanatory. | Or operator. Can also use `\|\|` |
| `la` | not (Arabic). Self-explanatory. | Not operator. Can also use `!` |

```text
niyya a = haqq;
niyya b = batil;

shart(a wa b) {
  // do something
} ilshart(la a) {
  // do something
} illa {
  // do something
}
```


### Loops
| Syntax | Meaning/Inspiration | Usage |
| :----: | :------------------ | :---- |
| `tawaf` | Circumbulate. The name for circumbulating around the Ka'bah for Hajj. Signifies going around and around. | For loop |
| `baynama` | While (Arabic). Self-explanatory | While loop |
| `iftar` | Breaking fast. The time when Muslims break fast is called `iftar`. | Break statement |
| `safar` | Journey or travel. Signifies the loop is going to continue on it's journey. | Continue statement |

```text
tawaf(niyya i = 0; i < 10; i = i + 1) {
  qul i; 
  // prints 0 to 9
}

niyya a = haqq;
niyya i = 0;
baynama(a) {
  shart (i < 10) {
    qul i;
    // prints 0 
    i = i + 1;
    safar;
  } illa {
    iftar;
  }
}
```

### Objects (Classes)
| Syntax | Meaning/Inspiration | Usage |
| :----: | :------------------ | :---- |
| `kitab` | Chapter (literal: Book) (Arabic). In Islamic books, chapters are called kitab. Classes are similar to chapters as they group related data together. | Class definition |
| `khalaq` | To create (Arabic). The constructor creates the class object. | Class constructor function |
| `nafs` | Self/Soul. Islamically, nafs is used to describe desires of the self. Represents the state of the instance (self). | Instance accessor (`this` in JavaScript, `self` in Python/Rust) | 


```text
kitab Animal {
  khalaq(name, sound) {
    nafs.name = name;
    nafs.sound = sound;
  }

  speak() {
    qul nafs.sound;
  }
}

kitab Feline > Animal {
  khalaq(name, sound) {
    super.khalaq(name, sound);
  }

  purr() {
    qul "purr"
  }
}

niyya cat = Feline("Hurayra", "Meow");
cat.speak();
// prints "meow"
cat.purr()
// prints "purr"
```

## Native Functions
I've implemented a few native functions to the program:
| Function Name | Parameters                                                                 | Return Type | Description                                                                             | 
| ------------- | -------------------------------------------------------------------------- | ----------- | --------------------------------------------------------------------------------------- |
| `clock`       |                                                                            | `number`    | Returns the time elapsed since the epoch in seconds                                     |
| `typeof`      | `arg: any`                                                                 | `string`    | Returns type of argument as a string                                                    |
| `str`         | `arg: any`                                                                 | `string`    | Converts the argument to a string                                                       |
| `str2num`     | `arg: string`                                                              | `number`    | Converts the argument to a number. Throws error if not possible.                        |
| `substr`      | `arg: string, start: number (positive int), length: number (positive int)` | `string`    | Returns a substring of the argument starting at `start` with length of `length`         |
| `index_of`    | `arg: string, substring: string`                                           | `number`    | Returns the index of the start of a substring in the argument. Returns -1 if not found. |
| `replace`     | `arg: string, old_substr: string, new_substr: string`                      | `string`    | Replaces all occurrences of `old_substr` in the argument with `new_substr`.             |
| `len`         | `arg: string`                                                              | `number`    | Returns the length of a string.                                                         |
| `max`         | `a: number, b: number`                                                     | `number`    | Returns the maximum of the inputs                                                       |
| `min`         | `a: number, b: number`                                                     | `number`    | Returns the minimum of the inputs                                                       |
| `pow`         | `base: number, exp: number`                                                | `number`    | Raises the base to the power of the exponent                                            |
| `random`      | `min: number, max: number`                                                 | `number`    | Returns a random number in the range of `min` to `max`                                  |
| `random_int`  | `min: number (int), max: number (int)`                                     | `number`    | Returns a random integer in the range of `min` to `max`                                 |