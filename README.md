# Rust Calculator

## Overview

This calculator was developed as an exploration into the Rust programming language, serving as both a learning experience and a practical application.

## Features

- Addition (+), Subtraction (-), Multiplication (*), and Division (/)

## How It Works

1. **User Input:**
   The calculator starts by soliciting input from the user, allowing them to perform basic arithmetic operations. The user can input expressions containing numbers and operators, such as `1 + 2 * 4 / 4`.

2. **Input Processing & Operator Verification:**
   The input is processed into a sequence of numbers and operators, using regular expressions to extract meaningful tokens. The calculator checks if the operators are correctly placed, preventing common input mistakes.

3. **Infix to Prefix Conversion:**
   To facilitate the correct order of operations, the calculator converts the infix expression to postfix notation.

4. **Calculation:**
   The final step involves evaluating the postfix expression to produce the result. The calculator uses a stack-based approach to handle numbers and operators, applying them in the order of operations.