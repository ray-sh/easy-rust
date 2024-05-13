# What are Slice Windows?

Slice windows involve breaking down a sequence or array into smaller contiguous subarrays or subsequences and performing operations on them. The window starts at the beginning of the sequence and moves forward one element at a time until it reaches the end. At each step, the window's size remains constant, but its position changes, allowing us to efficiently perform operations such as calculating sums, finding maximum or minimum values, or checking for specific patterns.

## Key Points to Remember

Define Window Size: Determine the size of the window based on the problem's requirements. The window size can vary depending on the problem, and choosing the right size is crucial for efficient computation.
Maintain Window State: Keep track of the current state of the window as it moves through the sequence. This includes keeping track of the window's start and end indices, as well as any intermediate results or data structures needed for calculations.

Handle Boundary Conditions: Handle edge cases and boundary conditions carefully, especially when the window reaches the beginning or end of the sequence. Adjust the window size or perform additional checks as necessary to handle these cases correctly.

Update Window: Update the window's state at each step by moving the window forward or adjusting its size based on the problem's requirements. This may involve adding or removing elements from the window and updating any relevant data structures or intermediate results.
Optimize Time Complexity: Aim for linear or near-linear time complexity by minimizing redundant calculations and efficiently updating the window's state. Look for opportunities to optimize the algorithm by avoiding unnecessary operations or using data structures such as hash maps or prefix sums to speed up computations.

## What's the advantage for rust in this case?

Slice Syntax: Rust's slice syntax (&[T]) makes it easy to work with contiguous portions of arrays or vectors. Slices provide a view into the underlying data without needing to copy it, which is efficient for passing data to functions or iterating over portions of it.

Pattern Matching: Rust's pattern matching capabilities, including match and if let, make it easy to handle different cases when working with slice windows. This allows for concise and readable code when checking boundary conditions or handling different states of the window.

Iterator and Iterator Adaptors: Rust's iterator trait (Iterator) and the associated methods provide powerful tools for working with sequences of data, including slices. Iterators allow for expressive and functional-style programming, making it easy to perform operations such as mapping, filtering, and folding over elements of a slice window.

Safety and Error Handling: Rust's strong emphasis on safety and error handling ensures that slice window code is robust and reliable. Rust's Result and Option types provide a convenient and ergonomic way to handle errors and edge cases, preventing unexpected behavior and runtime errors.

# How iter make code neat/efficiency in rust?

- [iter chain make code easy to read](https://docs.google.com/document/d/15Rd0dcggZcwJlCYsRg0BZhTlD8R5BCefqwn0exw7If4/edit?usp=sharing)

- iter chain is lazy by default, Each adapter in the chain (filter, map in the previous example) creates a new iterator, but it doesn't consume the entire underlying iterator immediately. You could call collect() to make the iter to eager. Also, that's usually the last step in the iter chain.

# Understand String/str/&str/&mut str

- [The difference](https://docs.google.com/document/d/1Ra30kWTH-sl8nTEsa-0dC4ggd8AHzhDnT-0ungQrqxE/edit)