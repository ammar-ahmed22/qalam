<div align="center">
<h1>qalam</h1>
<p>Islamic/Arabic terminology inspired, dead-simple, programming language</p>
</div>

## Introduction
As a learning exercise, to really hone my skills, I wanted to write a programming language myself. In order to accomplish this, I'm following the amazing tutorial called [Crafting Interpreters by Robert Nystrom](https://craftinginterpreters.com/).

While Robert goes through the entire process, step-by-step, to create a programming language he calls Lox, I didn't want to simply follow a tutorial and copy-paste code. I have found that real learning occurs when you change up tutorials a little so that you can have a deeper understanding.

In order to do this, I decided to create my own syntax for the language with the same general features as Lox. I also decided to use Rust to make the language instead of Java so that I cannot possibly copy-paste code.

## Using `qalam`
WIP

## Syntax
### Keywords
| Syntax | Meaning/Inspiration | Usage |
| :----: | :----------------- | :---: |
| `niyya` | Intention. Signifies the intention to store a value. | Variable declarations |
| `amal` | Good Deeds/Actions. Functions do things (actions/deeds) | Function declarations |
| `radd` | To return (Arabic) | Return statement |
| `qul` | To say (Arabic) | Print statement |

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
`shart` (condition) - if statement
`illa` (else) - else statement
`ilshart` - else if statement
`haqq` (truth) - boolean true
`batil` (falsehood) - boolean false
`wa` (and) - and operator (can also use &&)
`aw` (or) - or operator (can also use ||)
`la` (not) - not operator (can also use !)

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
`tawaf` (circumbulate) - for loop
`baynama` (while) - while loop
`iftar` (breaking fast) - break statement
`safar` (travel, journey) - continue statement

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
`kitab` (section, chapter) - class definition
`khalaq` (to create) - class constructor function

```text
kitab Animal {
  khalaq(name, sound) {
    this.name = name;
    this.sound = sound;
  }

  speak() {
    qul this.sound;
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