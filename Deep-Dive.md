---
geometry:

  - top=1in

  - bottom=1in

  - left=1in

  - right=1in

fontsize: 6pt  # Adjusted to a more standard size

header-includes:

  - \usepackage{sectsty}

  - \allsectionsfont{\centering}

  - \usepackage{fancyhdr}

  - \usepackage{etoolbox}  # Needed for patching commands

  - \AtBeginDocument{\begin{center}}  # Starts centering at the beginning

  - \AtEndDocument{\end{center}}      # Ends centering at the end
---

# Efficiency of the Sieve of Eratosthenes and Comparative Analysis of Prime-Finding Algorithms

** Note we may not want a table of contents when we generate the final pdf**

## Table of Contents

1. [Abstract](#abstract)

2. [Introduction](#introduction)

3. [History of the Sieve of Eratosthenes](#history-of-the-sieve-of-eratosthenes)

4. [Efficiency of the Sieve of Eratosthenes](#efficiency-of-the-sieve-of-eratosthenes)

5. [Other Prime-Finding Algorithms](#other-prime-finding-algorithms)

    1. [Algorithm 1](#Algorithm 1)

    2. [Algorithm 2](#Algorithm 2)

    3. [Algorithm 3](#Algorithm 3)

6. [Comparative Analysis](#comparative-analysis)

7. [Conclusion](#conclusion)

8. [References](#references)

---

## Abstract
*Provide a brief summary of the paper, including the main objectives, methods, key findings, and conclusions.*

## Introduction
*Introduce the topic of prime number algorithms, the importance of efficiency in computational number theory, and an overview of what the paper will cover.*

Example equ: The quadratic formula is given by $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$.

## History of the Sieve of Eratosthenes
*Discuss the origins of the Sieve of Eratosthenes, its historical significance, and its foundational role in the study of prime numbers.*

## Efficiency of the Sieve of Eratosthenes
*Analyze the computational complexity of the Sieve of Eratosthenes, including time and space requirements. Discuss its practical performance and any optimizations that have been developed.*

## Other Prime-Finding Algorithms
### Algorithm 1
#### Description
*Explain how the Sieve of Sundaram works and its methodology for finding prime numbers.*
#### Efficiency
*Analyze the computational complexity and practical efficiency compared to the Sieve of Eratosthenes.*

### Algorithm 2
#### Description
*Detail the Sieve of Atkin algorithm, highlighting its innovative approach to prime number generation.*
#### Efficiency
*Evaluate its performance metrics and compare its efficiency with other sieves.*

### Algorithm 3
#### Description
*Describe the trial division method for finding primes, including its simplicity and implementation.*
#### Efficiency
*Discuss the limitations in terms of efficiency, especially for large numbers, and compare it to sieve-based methods.*

## Comparative Analysis
*Compare and contrast the Sieve of Eratosthenes with the Sieve of Sundaram, Sieve of Atkin, and Trial Division in terms of efficiency, scalability, and practicality. Include tables or charts if necessary.*

## Conclusion
*Summarize the key findings of the paper, reiterate the efficiency of the Sieve of Eratosthenes, discuss the circumstances under which other algorithms may be more effective, and suggest potential areas for future research.*

## References
*List all the sources cited in the paper in the appropriate citation style.* Undetermined what style yet for citations

