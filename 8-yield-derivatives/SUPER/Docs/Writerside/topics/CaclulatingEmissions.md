# Calculating Emissions

In order to calculate the emission curve for SUPERy, I did math in python using defined parameters.

## Parameters

### Time-based Parameters
- Frequency of yield calculation, in this case, hourly
   ```tex
   f = \frac{24 \text{ periods}}{\text{day}}
   ```
- Length of token sale (How long can people buy SUPER for)
   ```tex
   \begin{align*}
   t_{\text{sale}} &= 7 \text{ days} \\
   &= 7 \text{ days} \times \frac{24 \text{ periods}}{\text{day}} \\
   &= 168 \text{ periods}
   \end{align*}
  ```
- Length of the development period (Period over which yield is distributed)
   ```tex
   \begin{align*}
   t_{\text{total}} &= 16 \text{ weeks} \\
   &= 16 \text{ weeks} \times \frac{7 \text{ days}}{\text{week}} \times \frac{24 \text{ periods}}{\text{day}} \\
   &= 2688 \text{ periods}
   \end{align*}
  ```
  
### SUPERy (Y) Parameters
- Max Supply 
   ```tex
   Y_{total} = 20,000,000 \ \text{SUPERy}
   ```
- Yield emitted during the token sale
  ```tex
  Y_{sale} = 6,000,000 \ \text{SUPERy}
   ```

## Before you start

List the prerequisites that are required or recommended.

Make sure that:
- First prerequisite
- Second prerequisite

## Part 1

Describe what the user will learn and accomplish in the first part,
then write a step-by-step procedure but on a real-world example.

1. Choose a "base" curve shape, for this, I chose the following curve, since it gives incentive to not only get in early,
   but also to not immediately claim yield, since the yield curve has a positive slope after the initial downfall:
   ```tex
   \begin{equation}
   x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
   \end{equation}
   ```

   ```bash
    run this --that
   ```

2. Step with a [link](https://www.jetbrains.com)

3. Final step in part 1.

## Part 2

This is the second part of the tutorial:

1. Step 1
2. Step 2
3. Step n

## What you've learned {id="what-learned"}

Summarize what the reader achieved by completing this tutorial.

<seealso>
<!--Give some related links to how-to articles-->
</seealso>
