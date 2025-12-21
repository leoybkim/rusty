# 8. Common Collections

rust's standard library includes a number of very useful data structure called *collections*. most other data types represent one specific value, but collections can contain multiple values. unlike the built-in array and tuple types, the data that these collectors point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. each kind of collection has different capabilities and costs:
- a *vector* allows you to store a variable number of values next to each other.
- a *string* is a collection of characters
- a *hash map* allows you to associate a value with a specific key. it's a particular implementation of the more general data structure called a *map*


## 8.2 storing lists of values with vectors

the first collection type is `vec<t>`, also known as a vector. vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. vectors can only store values of the same type.

### creating a new vector

to create a new, empty vector, we call the `vec::new` function

```rust
let v: vec<i32> = vec::new();
```

note that we added a type annotation because we aren't inserting any values into this vector but rust doesn't know what kind of elements we intend to store. vectors are implemented using generics. when we create a vector to hold a specific type, we can specify the type within angle brackets.

more often, you'll create a `vec<t>` with initial values, and rust will infer the type of value you want to store. rust conveniently provides the `vec!` macro, which will create a new vector that holds the values you give it. 

```rust
let v = vec![1, 2, 3];
```

### updating a vector
to create a vector and then add elements to it, we can use the `push` methods

```rust
    let mut v = vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

### reading elements of vectors

there are two ways to reference a value stored in a vector: via indexing or by using the `get` method. 

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("the third element is {third}");

    let third: option<&i32> = v.get(2);
    match third {
        some(third) => println!("the third element is {third}"),
        none => println!("there is no third element."),
    }
```

rust provides these two ways to reference an element so that you can choose how the program behaves when you try to use an index value outside the range of existing elements.

```rust
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // error!
    let does_not_exist = v.get(100);
```

the first `[]` mehthod will cause the program to panic, and this method is best used when you want your program to crash if there's an attempt to access an element past the end of the vector.

when `get` method is passed an index that is outside the vector, it returns `none` without panicking. you would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.

when the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure that this reference and any other references to the contents of the vector remain valid.

```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("the first element is: {first}"); // error!
```

adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where the vector is currently stored. in that case, the reference to the first element would be pointing to deallocated memory. the borrowing rules prevent programs from ending up in that situation.

### iterating over the values in a vector

to access each elements in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time.

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```

we can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.

```rust
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

### using an enum to store multiple types

the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum.

```rust
    enum spreadsheetcell {
        int(i32),
        float(f64),
        text(string),
    }

    let row = vec![
        spreadsheetcell::int(3),
        spreadsheetcell::text(string::from("blue")),
        spreadsheetcell::float(10.12),
    ];
```

if you don't know the exhaustive set of types a program will get at runtime to store in a vector, the enum techinique won't work. instead, you can use a trait object.

### dropping a vector drops its elements

like any other `struct`, a vector is freed when it goes out of scope.

```rust
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

when the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. the borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

## 8.2 storing utf-8 encoded text with strings

we discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

### defining strings

rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form, `&str`. in chapter 4, we talked about string slices, which are references to some utf-8 encoded string data stored elsewhere. string literals, for example, are stored in the program's binary and are therefore string slices. 

the `string` type, which is provided by rust's standard library rather than coded into the core langage, is a growable, mutable, owned, utf-8 encoded string type. when rustaceans refer to "strings" in rust, they might be referring to either the `string` or the string slice `&str` types, not just one of those types. although this section is largely about `string`, both types are used heavily in rust's standard library, and both `string` and string slices are utf-8 encoded.

### creating a new string
many of the same operations available with `vec<t>` are available with `string` as well because `string` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities. an example of a function that works the same way with `vec<t>` and `string `is the the `new` function to create an instance

```rust
    let mut s = string::new();
```

often we'll have some initial data with which we want to start the string. for that, we use the `to_string` method, which is available on any type that implements the `display` trait, as string literals do.

```rust
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
```



we can also use the function `string::from` to create a `string` from a string literal.

```rust
    let s = string::from("initial contents");
```

because strings are used for so many things, we can use many different generic apis for strings, providing us with a lot of options. some of them can seem redundant, but they all have their place! in this case, `string::from` and `to_string` do the same thing, so which one you choose is a matter of style and readability.

strings are utf-8 encoded, so we can include any properly encoded data in them

```rust
    let hello = string::from("السلام عليكم");
    let hello = string::from("dobrý den");
    let hello = string::from("hello");
    let hello = string::from("שלום");
    let hello = string::from("नमस्ते");
    let hello = string::from("こんにちは");
    let hello = string::from("안녕하세요");
    let hello = string::from("你好");
    let hello = string::from("olá");
    let hello = string::from("здравствуйте");
    let hello = string::from("hola");
```

### updating a string

a `string` can grow in size and its contents can change, just like the contents of a `vec<t>`, if you push more data into it. in addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `string` values.


#### appending with `push_str` or push

```rust
    let mut s = string::from("foo");
    s.push_str("bar");
```

the `push_str` method takes a string slice because we don't necessarily want to take ownership of the parameter.

```rust
    let mut s1 = string::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
```

if `push_str` method took ownership of `s2`, we wouldn't be able to print its value on the last line.

the `push` method takes a single character as a parameter and adds it to the `string`. 

```rust
    let mut s = string::from("lo");
    s.push('l');
```

#### concatenating with `+` or `format!`

often you'll want to combine two existing strings. one way to do so is to use the `+` operator.

```rust
    let s1 = string::from("hello, ");
    let s2 = string::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

the string `s3` will contain `hello, world!`. the reason `s1` is no longer valid after the addition, and the reason we used a reference to `s2`, has to do with the signature of the method that’s called when we use the `+` operator. the `+` operator uses the `add` method, whose signature looks something like this:

```rust
fn add(self, s: &str) -> string {
```

in the standard library, you'll see `add` defined using generics and associated types. here, we've substituted in concrete types, which is what happens when we call this method with `string` values.

the reason we're able to use `&s2` which is type of `&string`, not `&str` is that the compiler can coerce the `&string` argument into a `&str`. when we call the `add` method, rust uses a deref coercion, which here turns `&s2` into `&s2[..]`.

if we need to concatenate multiple strings, the behavior of the `+` operator gets unwieldy:
```rust
    let s1 = string::from("tic");
    let s2 = string::from("tac");
    let s3 = string::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

for combining strings in more complicated ways, we can instead use the `format!` macro:

```rust
    let s1 = string::from("tic");
    let s2 = string::from("tac");
    let s3 = string::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
```

the code generated by the `format!` macro uses referfences so that this call doesn't take ownership of any of its parameters.

### indexing into strings

in many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. however, if you try to access parts of a `string` using indexing syntax in rust, you’ll get an error


```rust
    let s1 = string::from("hi");
    let h = s1[0]; // error! the type `str` cannot be indexed by `{integer}`
```

rust strings don’t support indexing. but why not? to answer that question, we need to discuss how rust stores strings in memory

#### internal representation

a `string` is a wrapper over a `vec<u8>`.

```rust
    let hello = string::from("hola");
```

in this case, `len` will be `4`, which means the vector storing the string "hola" is 4 bytes long. each of these letters takes 1 byte when encoded in utf-8.

```rust
let hello = string::from("здравствуйте");
```

if you were asked how long the string is, you might say 12. in fact, rust’s answer is 24: that’s the number of bytes it takes to encode “здравствуйте” in utf-8, because each unicode scalar value in that string takes 2 bytes of storage. therefore, an index into the string’s bytes will not always correlate to a valid unicode scalar value.

```rust
let hello = "здравствуйте";
let answer = &hello[0]; // error!
```

when encoded in utf-8, the first byte of `з` is `208` and the second is `151`, so it would seem that answer should in fact be `208`, but `208` is not a valid character on its own. returning `208` is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that rust has at byte index 0. users generally don’t want the byte value returned, even if the string contains only latin letters: if `&"hi"[0]` were valid code that returned the byte value, it would return `104`, not `h`.

the answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

#### bytes, scalar values, and grapheme clusters

another point about utf-8 is that there actually three relevent ways to look at strings from rust's perspectives: as bytes, scalar values, and grapheme clustsers (the closest thing to what we would call *letters*).

if we look at the hindi word “नमस्ते” written in the devanagari script, it is stored as a vector of `u8` values that looks like this:
```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```


that’s 18 bytes and is how computers ultimately store this data. if we look at them as unicode scalar values, which are what rust’s `char` type is, those bytes look like this:

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

there are six `char` values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. 

finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the hindi word:
```rust
["न", "म", "स्", "ते"]
```

a final reason rust doesn’t allow us to index into a `string` to get a character is that indexing operations are expected to always take constant time (o(1)). but it isn’t possible to guarantee that performance with a `string`, because rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.


### slicing strings

indexing into string is often a bad idea because it's not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.

rather tahn indexing using `[]` with a single number, you can use `[]` with a range to create a string slice containing particular bytes.

```rust
let hello = "здравствуйте";

let s = &hello[0..4];
```

here `s` will be a `&str` that contains the first 4 bytes of the string. earlier, we mentioned that each of these characters was 2 bytes, which means `s` will be y`зд`.

if we were to try to slice only part of a character’s bytes with something like `&hello[0..1]`, rust would panic at runtime in the same way as if an invalid index were accessed in a vector.

you should use caution when creating string slices with ranges, because doing so can crash your program.

### iterating over strings

the best way to operate on pieces of strings is to be explicit about whether you want characters or bytes. for individual unicode scalar values, use the `chars` method. calling `chars` on "зд" separates out and returns two values of type `char`, and you can iterate over the result to access each element:
```rust
for c in "зд".chars() {
    println!("{c}");
}
```

this code will print the following:

```
з
д
```

alternatively, the `bytes` method returns each raw byte, which might be appropriate for your domain:

```rust
for b in "зд".bytes() {
    println!("{b}");
}
```
this code will print the 4 bytes that make up this string:



```
208
151
208
180
```

but be sure to remember that valid unicode scalar values may be made up of more than 1 byte.

getting grapheme clusters from strings, as with the devanagari script, is complex, so this functionality is not provided by the standard library. crates are available on crates.io if this is the functionality you need.

### handling the complexities of strings

to summarize, strings are complicated. different programming languages make different choices about how to present this complexity to the programmer. rust has chosen to make the correct handling of `string` data the default behavior for all rust programs, which means programmers have to put more thought into handling utf-8 data up front. this trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ascii characters later in your development life cycle.

## 8.3 storing keys with associated values in hash maps

the last of our common collections is the hash map. the type `hashmap<k, v>` stores a mapping of keys of the type `k` to values of type `v` using a *hashing function*, which determines how it places these keys adn values into memory. 

hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

### creating a new hash map

one way to create an empty hash map is to use `new` and to add elements with `insert`.

```rust
    use std::collections::hashmap;

    let mut scores = hashmap::new();

    scores.insert(string::from("blue"), 10);
    scores.insert(string::from("yellow"), 50);
```

note that we need to first `use` the `hashmap` from the collections portion of the standard library. of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude. hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.

just like vectors, hash maps store their data on the heap. this `hashmap` has keys of type `string` and values of type `i32`. like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.


### accessing values in a hash map

we can get a value out of the hash map by providing its key to the `get` method.

```rust
    use std::collections::hashmap;

    let mut scores = hashmap::new();

    scores.insert(string::from("blue"), 10);
    scores.insert(string::from("yellow"), 50);

    let team_name = string::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```

the `get` method returns an `option<&v>`; if there’s no value for that key in the hash map, `get` will return `none`. this program handles the `option` by calling `copied` to get an `option<i32>` rather than an `option<&i32>`, then `unwrap_or` to set `score` to zero if scores doesn’t have an entry for the key.

we can iterate over each key-value pair in a hash map in a similar manner as we do with vectors, using a `for` loop.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

This code will print each pari in an arbitrary order:
```
Yellow: 50
Blue: 10
```

### Managing Ownership in Hash Maps

For types that implements the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
```

We aren’t able to use the variables `field_name` and `field_value` after they’ve been moved into the hash map with the call to `insert`.

If we insert reference to values into the hash map, the values won't be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.


### Updating a Hash Map

Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time (but not vice versa). 

When you want to change the data in the hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key *doesn't* already have a value. Or you could combine the old value and the new value.

#### Overwriting a Value


If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
```

#### Adding a Key and Value Only If a Key Isn't Present

Hash maps have a special API for this called `entry` that takes the key you want to check as a parameter. The return value of the `entry` method is an enum called `Entry` that represents a value that might or might not exist. 

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
```

The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.


#### Updating a Value Based on the Old Value

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
```

The `split_whitespace` method returns an iterator over subslices, separated by whitespace, of the value in `text`. The `or_insert` method returns a mutable reference (`&mut V`) to the value for the specified key. Here, we store that mutable reference in the `count` variable, so in order to assign to that value, we must first dereference `count` using the asterisk (`*`). The mutable reference goes out of scope at the end of the `for` loop, so all of these changes are safe and allowed by the borrowing rules.

### Hashing Function

By default, `HashMap` uses a hashing function called *SipHash* that can porvide resistance to denial-of-service attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A *hasher* is a type that implements the `BuildHasher` trait.


