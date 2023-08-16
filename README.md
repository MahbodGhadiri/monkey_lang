# Monkey Lang

## ⚠ Disclaimer

This language is mostly based on the fantastic book
[Writing An Interpreter In Go](https://interpreterbook.com/)
This implementation of monkeyLang is for educational purpose only, as I
want to learn more about compilers and interpreters.
I am neither an experienced rust developer (as of Aug 16 2023) nor do I know anything about
developing a programming language, so this repo is most probably far from best-practices
and is not optimized at all.
This is just me learning new stuff.

## Why rust?

At this point you are probably asking why am I writing Interpreter in Rust using a book that uses Go.
The answer is how else can I write ⚡blazing fast⚡ and memory-safe code to implement a not so optimized and unsafe laguage?

well ... not really. I am just fascinated by Rust and I was waiting for the best excuse to learn rust. and what excuse is better than learning about Rust and Interpreters/Compilers at the same time?

## Features

the language is mostly gonna be based on the book, but may
also include more features e.g. more data types.

## Structure

#### Lexer

Given an input string, we first tokenize it with the help of
the Lexer. At this step we don't care about the syntax or the semantics
of our language. We only define what is considered an identifier, a keyword,
an operator, etc.

#### Parser

After tokenization, the Parser will basically check for the correctness of
the syntax. It reads through the tokens and builds an [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
The tree is an abstract summarization of our grammar for the given input. it is
abstract because it does not contain some inessential parts of the syntax like
punctuation(braces, brackets and etc.)

## Current State

| Feature        | State                                                 |
| -------------- | ----------------------------------------------------- |
| Lexer          | implemented basic features, more features to be added |
| Parser         | Not developed yet                                     |
| AST            | Not developed yet                                     |
| Eval/Semantics | Not developed yet                                     |
