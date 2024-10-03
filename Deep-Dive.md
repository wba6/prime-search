---
permalink: /
geometry:
  - top=1in
  - bottom=1in
  - left=1in
  - right=1in
fontsize: 4pt
documentclass: article
header-includes:
  - \usepackage{sectsty}
  - \allsectionsfont{\centering}
  - \usepackage{fancyhdr}
  - \usepackage{anyfontsize}
  - \usepackage{setspace}
  - |
    \makeatletter
    \renewcommand\normalsize{%
      \@setfontsize\normalsize{8pt}{10pt}%
      \abovedisplayskip 8pt
      \abovedisplayshortskip 4pt
      \belowdisplayshortskip 4pt
      \belowdisplayskip 8pt
      \abovedisplayskip 8pt
      \abovedisplayshortskip 4pt
      \belowdisplayshortskip 4pt
      \belowdisplayskip 8pt
    }
    \makeatother
---

# Efficiency of the Sieve of Eratosthenes and Comparative Analysis of Prime-Finding Algorithms

## Abstract

*Provide a brief summary of the paper, including the main objectives, methods, key findings, and conclusions.*

## Introduction

*Introduce the topic of prime number algorithms, the importance of efficiency in computational number theory, and an overview of what the paper will cover.*

Example equ: The quadratic formula is given by $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$.

## History of the Sieve of Eratosthenes

*Discuss the origins of the Sieve of Eratosthenes, its historical significance, and its foundational role in the study of prime numbers.*

One of the most popular prime number algorithms is the Sieve of Eratosthenes. This algorithm was founded by a famous Greek scientist named Eratosthenes of Cyrene. Eratosthenes had many talents, one of which was in mathematics. Mathematics during his life was nothing compared to modern-day mathematics; thus, it was easier to become talented at mathematics.

Eratosthenes discovered a systematic way to find primes. This system involved starting with a prime and then marking all multiples of that prime as composite. The numbers that do not get crossed off end up being the primes.

The Sieve of Eratosthenes has played a significant role in finding "small" primes back during the time when computers did not exist. Although better algorithms have been found to compute primes using computers, this algorithm is one that every math or computer science student should be taught.

## Efficiency of the Sieve of Eratosthenes

*Analyze the computational complexity of the Sieve of Eratosthenes, including time and space requirements. Discuss its practical performance and any optimizations that have been developed.*

## Other Prime-Finding Algorithms

### Trial Division

#### Description

*Explain how the Trial Division algorithm works and its methodology for finding prime numbers.*

The Trial Division algorithm is typically the easiest to understand prime number algorithm. This algorithm aims to determine if a number can be factored compared to how the Sieve of Eratosthenes works by eliminating multiples of primes. There is a strategic strategy to determine the possible factors of a number. The possible factors of a number turn out being all numbers less than the square root of said number

The algorithm goes as follows:
+ Take in a positive integer n
+ Determine the range of numbers you need to check as possible factors
+ Iterate through the range of possible factors
+ If the factor divides evenly into the number, then it is composite
+ If all possible factors do not divide evenly into the number, then it is prime.


#### Efficiency

*Analyze the computational complexity and practical efficiency compared to the Sieve of Eratosthenes.*

**Dont know if I should include this here or in the description:** Trial Division algorithm is efficient is determining if single numbers are prime; however, the Sieve of Eratosthenes is efficient is finding all the primes in a specified limit. A main trade off for the Trial Division algorithm is that 

#### Description

*Detail the Sieve of Atkin algorithm, highlighting its innovative approach to prime number generation.*

#### Efficiency

*Evaluate its performance metrics and compare its efficiency with other sieves.*

### Miller-Rabin Primality Test

#### Description

*Describe the Miller-Rabin Primality Test method for finding primes, including its simplicity and implementation.*

#### Efficiency

*Discuss the limitations in terms of efficiency, especially for large numbers, and compare it to sieve-based methods.*

## Comparative Analysis

*Compare and contrast the Sieve of Eratosthenes with the Sieve of Sundaram, Sieve of Atkin, and Trial Division in terms of efficiency, scalability, and practicality. Include tables or charts if necessary.*

## Conclusion

*Summarize the key findings of the paper, reiterate the efficiency of the Sieve of Eratosthenes, discuss the circumstances under which other algorithms may be more effective, and suggest potential areas for future research.*

## References

*List all the sources cited in the paper in the appropriate citation style.* Undetermined what style yet for citations
https://medium.com/geekculture/sieve-of-eratosthenes-one-of-the-oldest-algorithms-still-prevalent-as-if-it-were-born-yesterday-c3e854df5dc6 - provides a few optimizations for the sieve of Eratosthenes

