# Unix Philosophy Reference Guide

## Core Tenets (Doug McIlroy, 1978)

The original Unix philosophy from Bell System Technical Journal:

### 1. Do One Thing Well

**"Make each program do one thing well. To do a new job, build afresh rather than complicate old programs by adding new features."**

- Write focused, single-purpose programs
- Avoid feature creep and bloat
- Build new tools instead of overloading existing ones
- Simplicity enables maintainability and understanding

### 2. Composability

**"Expect the output of every program to become the input to another, as yet unknown, program."**

- Design for pipes and composition
- Don't clutter output with extraneous information
- Avoid stringently columnar or binary input formats
- Don't insist on interactive input
- Use text streams as the universal interface

### 3. Build Early, Iterate Often

**"Design and build software, even operating systems, to be tried early, ideally within weeks. Don't hesitate to throw away the clumsy parts and rebuild them."**

- Prototype quickly
- Test early and often
- Be willing to discard and rebuild
- Iterate based on real-world use

### 4. Use Tools Over Manual Labor

**"Use tools in preference to unskilled help to lighten a programming task, even if you have to detour to build the tools and expect to throw some of them out after you've finished using them."**

- Automate repetitive tasks
- Build throwaway tools when needed
- Invest in tooling even for one-time use
- Tools compound in value over time

## The Three-Sentence Summary (McIlroy, 2003)

> "Write programs that do one thing and do it well.  
> Write programs to work together.  
> Write programs to handle text streams, because that is a universal interface."

## Eric S. Raymond's Extended Rules

From *The Art of Unix Programming* (2003):

### Rule of Modularity

Write simple parts connected by clean interfaces.

### Rule of Clarity

Clarity is better than cleverness.

### Rule of Composition

Design programs to be connected to other programs.

### Rule of Separation

Separate policy from mechanism; separate interfaces from engines.

### Rule of Simplicity

Design for simplicity; add complexity only where you must.

### Rule of Parsimony

Write a big program only when it is clear by demonstration that nothing else will do.

### Rule of Transparency

Design for visibility to make inspection and debugging easier.

### Rule of Robustness

Robustness is the child of transparency and simplicity.

### Rule of Representation

Fold knowledge into data so program logic can be stupid and robust.

### Rule of Least Surprise

Do the least surprising thing.

### Rule of Silence

When a program has nothing surprising to say, it should say nothing.

### Rule of Repair

Repair what you can — but when you must fail, fail noisily and as soon as possible.

### Rule of Economy

Programmer time is expensive; conserve it in preference to machine time.

### Rule of Generation

Avoid hand-hacking; write programs to write programs when you can.

### Rule of Optimization

Prototype before polishing. Get it working before you optimize it.

### Rule of Diversity

Distrust all claims for "one true way".

### Rule of Extensibility

Design for the future, because it will be here sooner than you think.

## Kernighan & Pike's Philosophy (1984)

From *The Unix Programming Environment*:

> "The power of a system comes more from the relationships among programs than from the programs themselves. Many UNIX programs do quite trivial things in isolation, but, combined with other programs, become general and useful tools."

Key principles:

- Software tools over monolithic systems
- Easy to use individually, easier to combine
- Programs fit into the environment
- Design for combination, not just internal structure

## Practical Applications

### Text Streams as Universal Interface

- Line-based text is easy to process
- No special parsers needed
- Works with pipes, redirects, filters
- Human-readable for debugging
- Example: `cat file.txt | grep "error" | wc -l`

### Small, Composable Tools

**Good Examples:**

- `ls` - list files
- `grep` - search text
- `wc` - count words/lines
- `sort` - sort lines
- `uniq` - remove duplicates
- `cut` - extract columns
- `sed` - stream editor
- `awk` - pattern processing

**Composition Example:**

```bash
# Find the 10 most common words in a file
cat file.txt | \
  tr '[:upper:]' '[:lower:]' | \
  tr -s ' ' '\n' | \
  sort | \
  uniq -c | \
  sort -rn | \
  head -10
```

### Clean Interfaces

- Standard input/output/error
- Exit codes (0 = success, non-zero = failure)
- Simple, predictable behavior
- Minimal side effects
- No hidden state

## Modern Interpretations

### For Modern Development (Isaac Z. Schlueter, npm creator)

- Write modules that do one thing well
- Write a new module rather than complicate an old one
- Write modules that encourage composition rather than extension
- Write modules that handle data streams
- Write modules that are agnostic about their input/output sources

### When to Break the Rules

The Unix philosophy is a guide, not a law. Consider breaking it when:

1. **User Experience Suffers**: Sometimes users want integrated tools, not composition
2. **Performance Demands It**: Tight integration can be faster than multiple processes
3. **Complexity is Necessary**: Some problems are inherently complex
4. **Platform Constraints**: Not all environments support pipes/text streams well

> "Abandon a standard when it is demonstrably harmful to productivity or user satisfaction." — Jef Raskin

### Common Criticisms

**The `ls` Problem**: Even `ls` has 35+ options and complex backward compatibility. Users prefer feature-rich tools over composing many small ones.

**Conservation of Complexity**: Complexity doesn't disappear—it moves. Unix philosophy says "put complexity in shell scripts, not compiled programs." But where should the complexity go?

**Modern Data**: Binary formats (images, video, executables) don't fit the text stream model.

## Key Takeaways for CLI Development

### DO

- Write focused, single-purpose tools
- Accept input from stdin, write to stdout
- Use text as the default format
- Provide clean, simple interfaces
- Make tools composable via pipes
- Fail fast and clearly
- Build for early testing
- Automate with tools

### DON'T

- Add features indefinitely to one program
- Require interactive input when piped
- Clutter output with metadata/formatting
- Use binary/proprietary formats unnecessarily
- Build monolithic "do-everything" programs
- Make tools interdependent
- Hide errors or state
- Optimize prematurely

## The Philosophy in One Image

``` text
Program A --[text]--> Program B --[text]--> Program C --[text]--> Result

Each program:
- Does one thing
- Reads text from stdin
- Writes text to stdout  
- Can be replaced independently
- Combines with unknown future programs
```
