<h1><div align="center">Recursion</div></h1>
<div align="center">Example of recursive functions using Rust</div>
<br>
Recursion is a technique used to solve computer problems by creating a function that calls itself until your program achieves the desired result.

<br>

A recursive function consists of two parts: 
<ol>
    <li>The recursive call
    <li>The condition that stops a recursive function from calling itself know as the base case.
</ol>

<br>

The base case is the condition that checks to see if the condition you desired has been achieved.  Every recursive should have at least one base case.

<br>

 The base case within the examples [here](https://github.com/nagashi/recursion/blob/main/src/main.rs), for the  count_down function is when nbr is less than 1.  In the num_range function, the base case is when begin_num is larger than end_num.  The recursive call is when the function calls itself, which adds to the recursice call stack.

 <br>

The call stack is at the heart of the recursive function. It keeps track of each time the function is called. When the call stack hits a RETURN, it pops the current function off the stack and goes back to whichever function's now on top.  So…when count_down(10) calls count_down(1), it's going to wait until count_down(1) ends before it can also end. Once count_down(1) ends, count_down(10) finishes its run. See sample output for the count_down(10) function below: 

<br>

<div align="center">Countdown [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]</div>

<br>

Pros:
<ol>
    <li>Recursion adds clarity and (sometimes) reduces the time needed to write and debug code (but doesn't necessarily reduce space requirements or speed of execution).
    <li>Reduces time complexity.
    Performs better in solving problems based on tree structures.
    <li>It is a unique way of implementing a variable number of nested loops/iterations (and the only elegant way of implementing a big constant number of nested loops).
</ol>

Cons:
<ol>
    <li>It is usually slower due to the overhead of maintaining the stack.
    <li>It usually uses more memory for the stack.
    <li>Recursive methods will often throw a StackOverflowException when processing big sets. Recursive loops/iterators don't have this problem.
</ol>

<br>
<br>

Thanks for reading and do reach out and let me know if you have any questions or concerns.&nbsp;&nbsp;Click 'Star' if you like the program.&nbsp;&nbsp;All suggestions, constructive, even non-constructive, will be welcomed.<img src = "images/ok.png" alt="Image denoting Ok"
          width="30"
          height="20"
          border="0"
        />

<br>
<br>
<div align="center"> 

[![MIT licensed][mit-badge]][mit-url]&nbsp;&nbsp;[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

</div>
<br>
<br>
<p>

### License

This project is licensed under the&nbsp;[MIT license](LICENSE).

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in 'Recursion' by you, shall be licensed as MIT, without any additional terms or conditions.

</p>


