% Let's build a binary tree!

Let's build a binary tree of strings in Rust. To recap, each node in a binary
tree:

1. must have a value
2. may or may not have left and/or right children

So we can describe a single node like this:

```ignore
struct Node {
    val: &str,
    l: Option<Node>,
    r: Option<Node>,
}
```

Unfortunately the above won't compile! The first issue is explained in the Rust
book - if a struct will contain borrowed values (`val`), we must tell the
compiler how long they're expected to last. The borrow checker can then enforce
that any use of the `val` field obeys the restriction. The second issue is simply
that Rust cannot know how much memory to allocate to the data type if it
recursively contains itself. The fix is simply to fill this field with a
pointer instead of an instance of the type. Our fixed Node:

```rust
struct Node<'a> {
    val: &'a str,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}
```

Now we know what our trees are made up of, but we have no way to build one.
Let's create a method to allow insertion of items into a tree. My preference
as a rust beginner is to write down the Rust-like pseudocode (i.e. minus
mutability and lifetime specifiers) of the logic I'd like to use, then convince
the borrow checker that my code is correct.

```ignore
impl Node {
    pub fn insert(&self, new_val: &str) {
        if self.val == new_val {
            return
        }
        let target_node = if new_val < self.val { &self.l } else { &self.r };
        match target_node {
            &Some(subnode) => subnode.insert(new_val),
            &None => {
                let new_node = Node { val: new_val, l: None, r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}
```

There's nothing exciting going on here. We find which side of the current
Node the new value should go on, then either add it as a subnode if there isn't
one already on that side, or recurse down by calling insert on the subnode.

Needless to say, the borrow checker finds this piece of code very offensive!
Since the first compiler complaints are about lifetimes, let's work
those out first.

The first error we get is pretty clear (`wrong number of lifetime parameters`)
and easily fixed, right? We're doing an `impl` for `Node`, but our type is
actually `Node<'a>`, so tweak that and the problem is solved? Unfortunately not.

Let's consider at a bit of background here. Looking back at the section on
'Ownership' in the rust book, we can see that when creating functions we have
to *declare* the lifetimes as a parameter of the function to bring those
lifetime names in scope. This is the key - whenever a lifetime is referenced, it
has been declared somewhere beforehand. In fact, this is what we did for the
struct when saying that `val` would last for the lifetime `'a`.

Coming back to the problem at hand, we know know we need to bring the lifetime
into scope somehow. The correct way to do this in an impl is as a parameter to
the impl itself, like so:

```ignore
impl<'a> Node<'a> {
```

If we put this in, we now get an error about not being able to infer an
appropriate lifetime for new_val when creating a new Node. There's a temptation
when seeing an error like this to shove in a lifetime as a parameter to the
function, add it to the function argument and hope for the best. In fact, a lot
of the time this does work.

Unfortunately you need to take more care when implementing functions for an
impl, as there's already a lifetime hanging around - the lifetime for the entire
struct. Looking back at the definition of a Node, we've said that the value and
sub-Nodes of a Node (and therefore all their values as well) must have the same
lifetime as the Node itself. The error is about the value of a sub-Node, so
let's make sure it has the same lifetime as the parent Node.

```ignore
    pub fn insert(&self, new_val: &'a str) {
```

We've used the lifetime brought in scope by the impl, used as the lifetime of
the struct the impl is for, as the lifetime of the value. 

Hooray! We've completed a first pass of convincing the borrow-checker that all
the values involved live for the correct amount of time. The next stage is to
make sure that we can actually modify values, as Rust requires us to explicitly
keep track of mutability.

We get two different types of error if we try to compile now. The first talks
about moving values out of a borrow - when we assigned the value of
`target_node`, the `&` operator was used so we took a reference. The pattern
we match on attempts to take ownership (i.e. 'move') the value inside the
target_node option...which isn't allowed, because we're just borrowing it! The
Rust book chapter on patterns mentions the way forward here: `ref`.

```ignore
            &Some(ref subnode) => subnode.insert(new_val),
```

The next error relates to `target_node`. We want to replace the value being
pointed to (`*target_node = ...`), so we need to make the
reference we take be mutable. This has the knock on effect of needing to
change our pattern matching, as `mut` is technically part of the type.

```ignore
        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
        match target_node {
            &mut Some(ref subnode) => subnode.insert(new_val),
            &mut None => {
```

Attempting to compile now tells us that we can't take a mutable reference to
`self.l` or `self.r` as `self` isn't actually mutable. We fix this in the
arguments to the function:

```ignore
    pub fn insert(&mut self, new_val: &'a str) {
```

As is typical when fixing an error by adding mutability or lifetime constraints,
changes are required elsewhere to comply with the new rules. In this case, our
`ref subnode` is not mutable, so you can't call `subnode.insert` (which now
requires `self` to be mutable). Again referring back to the patterns section
of the Rust book, we find our answer:

```ignore
            &mut Some(ref mut subnode) => subnode.insert(new_val),
```

It's worth noting that this has worked because we're already taking a
mutable reference to the `Option` - if that was immutable, so would the sub-Node
inside be.

You should now find that compilation is successful! The completed code is below
along with a few test cases to see it working.

```rust
#[derive(PartialEq)]
struct Node<'a> {
    val: &'a str,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}
impl<'a> Node<'a> {
    pub fn insert(&mut self, new_val: &'a str) {
        if self.val == new_val {
            return
        }
        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node { val: new_val, l: None, r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}
fn main () {
    let mut x = Node { val: "m", l: None, r: None };
    x.insert("z");
    x.insert("b");
    x.insert("c");
    assert!(x == Node {
        val: "m",
        l: Some(Box::new(Node {
            val: "b",
            l: None,
            r: Some(Box::new(Node { val: "c", l: None, r: None })),
        })),
        r: Some(Box::new(Node { val: "z", l: None, r: None })),
    });
}
```

In the next instalment we'll look at implementing an iterator and why putting
`'a` as a lifetime everywhere can be somewhat limiting.