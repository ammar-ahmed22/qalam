<div align="center">
<h1>qalam</h1>
<p>Islamic/Arabic terminology inspired, dead-simple, interpreted, programming language</p>
</div>

## Table of Contents
- [Introduction](#introduction)
- [Using `qalam`](#using-qalam)
- [Syntax](#syntax)
  * [Keywords](#keywords)
  * [Conditionals](#conditionals)
  * [Loops](#loops)
  * [Objects (Classes)](#objects--classes-)
- [Types](#types)
- [Native Functions](#native-functions)
- [Complete Example](#complete-example)
- [Speed](#speed)

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
| `shai` | Thing/Object (Arabic). Variables can store any thing/object. | Variable declarations |
| `amal` | Good Deeds/Actions. Functions do things (actions/deeds) | Function declarations |
| `radd` | To return (Arabic) | Return statement |
| `qul` | To say (Arabic) | Print statement |
| `ghaib` | Unseen, hidden. Signifies a value is not present | Null value |

```text
shai a = 1;
shai b = 2;

amal add(a, b) {
  radd a + b;
}

qul add(a, b);
// prints 3
```

### Conditionals
| Syntax | Meaning/Inspiration | Usage |
| :----: | :------------------ | :---- |
| `itha` | If (Arabic). Self-explanatory | If statement |
| `illa` | Else (Arabic). Self-explanatory | Else statement |
| `haqq` | Truth. A boolean `true` is the epitome of truth. | Boolean `true` value |
| `batil` | Falsehood. A boolean `false` is the epitome of falsehood. | Boolean `false` value |
| `wa` | and (Arabic). Self-explanatory. | And operator. Can also use `&&` |
| `aw` | or (Arabic). Self-explanatory. | Or operator. Can also use `\|\|` |
| `la` | not (Arabic). Self-explanatory. | Not operator. Can also use `!` |

```text
shai a = haqq;
shai b = batil;

itha(a wa b) {
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

shai a = haqq;
shai i = 0;
baynama(a) {
  itha (i < 10) {
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
| `ulya` | Most elevated, superior (Arabic). The superclass is superior to the subclass. | Superclass accessor (`super` in JavaScript) |
| `ibn` | Son of/Child of (Arabic). Subclasses inherit from the superclass as children inherit from parents | Class inheritance operator |  

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

kitab Feline ibn Animal {
  khalaq(name, sound) {
    ulya.khalaq(name, sound);
  }

  purr() {
    qul "purr"
  }
}

shai cat = Feline("Hurayra", "Meow");
cat.speak();
// prints "meow"
cat.purr()
// prints "purr"
```

## Types
Below are the built-in types supported by `qalam`:
| Type     | Description                                                                                                                      | Initialization Example          |
| -------- | -------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- |
| `number` | Numerical value. All numbers are stored as floating point values. Numbers without a fractional part are considered as integers.  | `niyya num = 1.0;`              |
| `string` | Collection of characters. Characters can be indexed with integers. Initialize with double quotes.                                | `niyya name = "Ammar";`         |
| `bool`   | Boolean true or false. `haqq` = true, `batil` = false.                                                                           | `niyya is_foo = haqq;`          |
| `array`  | Collection of any values. Values can be indexed and set with integers. Initialize with square braces.                            | `niyya arr = [1, "one", haqq];` |

## Native Functions
I've implemented a few native functions to the program:
| Function Name | Parameters                                                                 | Return Type | Description                                                                                  | 
| ------------- | -------------------------------------------------------------------------- | ----------- | -------------------------------------------------------------------------------------------- |
| `clock`       |                                                                            | `number`    | Returns the time elapsed since the epoch in seconds                                          |
| `typeof`      | `arg: any`                                                                 | `string`    | Returns type of argument as a string                                                         |
| `str`         | `arg: any`                                                                 | `string`    | Converts the argument to a string                                                            |
| `str2num`     | `arg: string`                                                              | `number`    | Converts the argument to a number. Throws error if not possible.                             |
| `substr`      | `arg: string, start: number (positive int), length: number (positive int)` | `string`    | Returns a substring of the argument starting at `start` with length of `length`              |
| `index_of`    | `arg: string, substring: string`                                           | `number`    | Returns the index of the start of a substring in the argument. Returns -1 if not found.      |
| `replace`     | `arg: string, old_substr: string, new_substr: string`                      | `string`    | Replaces all occurrences of `old_substr` in the argument with `new_substr`.                  |
| `len`         | `arg: string \| array`                                                     | `number`    | Returns the length of a string or array.                                                     |
| `max`         | `a: number, b: number`                                                     | `number`    | Returns the maximum of the inputs                                                            |
| `min`         | `a: number, b: number`                                                     | `number`    | Returns the minimum of the inputs                                                            |
| `pow`         | `base: number, exp: number`                                                | `number`    | Raises the base to the power of the exponent                                                 |
| `random`      | `min: number, max: number`                                                 | `number`    | Returns a random number in the range of `min` to `max`                                       |
| `random_int`  | `min: number (int), max: number (int)`                                     | `number`    | Returns a random integer in the range of `min` to `max`                                      |
| `push`        | `arr: array, val: any`                                                     | `ghaib`     | Pushes a value to the end of an array                                                        |
| `pop`         | `arr: array`                                                               | `any`       | Pops a value from the end of the array and returns it. If it does not exist, returns `ghaib` |
| `Array`       | `size: number (positive int), value: any`                                  | `array`     | Creates an array of `size` values all initialized to `value`.                                |
| `code`        | `char: string`                                                             | `number`    | Returns the character code for a single character string.                                    |
| `floor`       | `num: number`                                                              | `number`    | Returns the closest integer less than or equal to `num`                                      |
| `ceil`        | `num: number`                                                              | `number`    | Returns the closest integer greater than or equal to `num`                                   |
| `round`       | `num: number`                                                              | `number`    | Returns the closest to `num`. If `num` is halfway between 2 integers, returns away from 0.   |

## Complete Example
To showcase the functionalty of `qalam`, I've provided an example below of reversing a linked list:

```
kitab ListNode {
  khalaq(value, next) {
    nafs.value = value;
    nafs.next = next;
  }
}

amal list_to_string(head) {
  shai string = "";
  shai curr = head;
  baynama(curr != ghaib) {
    string += str(curr.value);
    itha(curr.next) {
      string += " -> ";
    }
    curr = curr.next; 
  }

  radd string;
}

shai list = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5, ghaib)))));

amal reverse_list(head) {
  shai prev = ghaib;
  shai curr = head;
  shai next = ghaib;
  baynama(curr != ghaib) {
    next = curr.next;
    curr.next = prev;
    prev = curr;
    curr = next;
  }

  radd prev;
}

qul "Original list:";
qul list_to_string(list);
qul "Reversed list:";
qul list_to_string(reverse_list(list));
```

### Output
```
Original list:
1 -> 2 -> 3 -> 4 -> 5
Reversed list:
5 -> 4 -> 3 -> 2 -> 1
```

### More Examples
I've provided many more examples, in the [examples directory](./examples). You can run them with `cargo run --example <name>` or by running the `main.qlm` file inside the examples subdirectory with `qalam <file_path>`. 

## Speed
`qalam` is an interpreted language with dynamic types. However, since there are essentially zero optimizations done, it is extremely slow. I am also an extreme amateur when it comes to Rust programming, therefore, it is probably even slower than Robert's implementation in Java as I definitely did a million things wrong when using Rust. 

In order to showcase how slow it actually is, I'll compare it's speed to JavaScript and Python (interpreted, dynamically typed languages). For the comparison, I'll use the calculation of the 30th fibonacci number using a recursive algorithm. 

Below are the scripts for each language
### Python
```python
import time

def fib(n):
  if n <= 1:
    return n
  else:
    return fib(n - 1) + fib(n - 2)

start = time.time()
result = fib(30)
end = time.time()
print(f"{end - start}")
```

### JavaScript
```javascript
function fib(n) {
  if (n <= 1) {
    return n;
  } else {
    return fib(n - 1) + fib(n - 2);
  }
}

let start = Date.now() / 1000;
let result = fib(30);
let end = Date.now() / 1000;
console.log(`${end - start}`);
```

### Qalam
```
amal fib(n) {
  itha(n <= 1) {
    radd n;
  } illa {
    radd fib(n - 1) + fib(n - 2);
  }
}

shai start = clock();
shai result = fib(30);
shai end = clock();
qul(str(end - start));
```

Each test was run 10 times and the average runtime is tabulated below:
| Language | Average Runtime (s) | 
| -------- | --------------- | 
| JavaScript | 0.0185 |
| Python | 0.2238 |
| Qalam | 36.2470 |

JavaScript and Python take less than second each. Qalam takes half a minute. It is extremely slow, however, I am still proud that it works lol.