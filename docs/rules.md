## Rules

This outlines the guidelines I wish to follow in developing Dash.

### Rules for Language Features

At the time of writing, the language is still new enough to where I can still add needed features. If anything, it really needs them. I've been spending a lot of time on the types, so it really needs constructs. When adding new constructs, I wish to follow these guidelines:

* No syntactic sugar. Real programming is not having so much syntactic sugar you don't have to think.
* Limit abstractions. Abstractions are different from syntactic sugar. Certain constructs such as strings, arrays, SIMD, and threading should be abstracted away, but not to the point to where you can't look at it and have a good idea of how its supposed to translate.
* Each construct must have a purpose and must have a semi-obvious translation to the assembly. That means I don't want 20 different styles of loops.
* Keywords are okay. I don't mind verbose languages- I actually really like how the Ada language approaches a lot of things. Each symbol should serve one purpose, but we shouldn't adopt keywords as a last resort.
* No nested expressions. Readability matters Each line should represent one expression. Some languages allow things like variable assignments in the loop controls. Except for things like for-loops, I think this is kind of a bad idea.
* Variable types must be obvious. If I have to work my way up the code and trace a variable, the language is doing it wrong. I have had to actually do this. Python comes to mind...

The most important thing to avoid is bloat. I don't want to follow the "everything but the kitchen sink" model that languages like C++, D, and many others try (I actually think D includes the kitchen sink and whole showroom). Good programming is not knowing one language and using it for everything. Good programming is knowing several languages, and choosing whichever one is best for your problem at the moment. 

### Inspiration

I am open to ideas from any language. For me, my favorite languages are C, C++, Rust, Haskell, and Ada. Even though I don't use Ada much, I really really like it in concept, so I'd like to pull ideas from there first. When it comes to hardware, I think C does a lot of things right and makes sense if you think like a computer (quoting Linus Torvalds...) so I also use this as inspiration.

