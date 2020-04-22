
# Beginner's Guide To SIMD

Hello and welcome to our SIMD basics guide!

Because SIMD is a subject that many programmers haven't worked with before, we thought that it's best to outline some terms and other basics for you to get started with.

## Quick Background

**SIMD** stands for *Single Instruction, Multiple Data*. In other words, SIMD is when the CPU performs a single action on more than one logical piece of data at the same time. Instead of adding two registers that each contain one `f32` value and getting an `f32` as the result, you might add two registers 