## Why Rust for the compiler?

When I started learning about compilers, I used C++. And that was natural choice; C++ lends itself well to compiler development (think LLVM...), and for me personally, I had been using C++ almost daily for six years, so I wanted to focus on the concept and not worry about learning the language.

However, I feel that C++ presents issues. It has pointers and references, which is a useful feature in the right situations. And I do not think compiler development is always the right situation. I was getting tired of the segmentation faults and random crashes which can be really hard to debug. And while C++'s OOP is really good for the trees needed in compiler development, the language lacks (in my opinion) features that are good for compiler development, pattern matching being one. 

I experimented with a few other languages, and eventually settled on Rust. I think Rust is extremely well suited for compiler development. Its memory model is extremely safe- I've never had a crash. Its enforced error handling is amazing. In fact, Rust is so safe I don't think I've ever had a significant crash with it- I don't think I've really ever had one at all. Usually if there's a compiler issue, it shows up in the assembly stage. Rust has fabulous pattern matching, I love how you can attach values to enums, and even though it doesn't have OOP in the sense of C++/Java/etc, I actually like the structure I came up with better. Finally, Rust runs on pretty much any platform and architecture.

