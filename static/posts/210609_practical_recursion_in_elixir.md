---
title: Practical recursion in Elixir
date: "2021-06-09"
author: Gertjan Assies
category: code
tags: elixir, recursion
image: "/static/images/practical_recursion_top.jpg"
summary: Explaining recursion with a very practical use case
published: true

---

Usually, when the new year starts, a lot of people are starting their "new years resolutions", which usually are things like exercise more, eat healthier, which non-quantitive nature are easy to postpone or avoid, therefore I rather set goals.

I have an electric bike that keeps track of the total km's cycled, in 2018 I cycled ~5000 km, in 2019 ~3000 km, so a downward trend was showing.

One of my (modest) goals for 2020 is to cycle at least 4000km which is around 80 km a week (my daily commute is around 35km), so if I cycle to work at 2 to 3 times a week, I will make it easily.

Now to help keep track, I wrote a small command-line tool, that allows me to enter the current kilometres, and predict using linear trend analysis where it would end up at the end of the year.

Linear trend analysis takes the average change between measurements and then apply that average to a future value $x$. In formula form this is

$$ \frac{\frac{v_x-v_n}{t_x-t_n}}{1} = \frac{\frac{v_1-v_0}{t_1-t_0} + \frac{v_2-v_1}{t_2-t_1} \dots \frac{v_n-v_{n-1}}{t_n-t_{n-1}}}{n}
$$

or .. the average of all the previous points (0 to n) is equal to the average of the last value (n) and the future value (x) divided by the number of days from the last value to that future date.

so what we need is a recursive function that does this part:

$$ \frac{v_1-v_0}{t_1-t_0} + \frac{v_2-v_1}{t_2-t_1} \dots \frac{v_n-v_{n-1}}{t_n-t_{n-1}}
$$

I store it as a JSON data structure, so a simple map with dates and km values

```elixir
%{"2020-01-01" => 8091, "2020-01-10" => 8119, "2020-01-13" => 8166, "2020-01-16" => 8197}
```

Which gets translated as a list of structs as sorting on the date is needed

```elixir
deltas = loop([], nil, measurements)

  defp loop(acc, prev, list) do
    case list do
      [] -> acc
      [head | tail] when prev == nil -> loop(acc, head, tail)
      [head | tail] ->
        delta = (head.value - prev.value) / diff_string_date(prev.date, head.date)
        loop([delta | acc], head, tail)
    end
  end
```

* We start with an empty accumulator, no previous value and the list of km's cycled
* when we don't have a previous value, we call ourselves with the head of the list as the previous value and the tail as the list
* when we have a previous value we calculate the average km/days between the previous and the head of the list, add it to the accumulator and call ourselves again with the head as the previous value and the tail
* when we call it with an empty list we return the accumulator (exit condition)

A big thing in recursion is tail-recursive or tail-call optimization.

Normally when you call function B from function A the current state of function A will be put on the stack end retrieved again when Function B returns, except when the call to function B is the last thing function A does. then it only has to return the result of function B

When calling a recursive function with a large list, this could cause the stack to grow beyond its memory allowance and crash your application, solving this is called 'Tail call optimization' and just means we need to make sure the recursive call is the last statement in the function.

We can see from the code that we call ourselves from 2 places, so how can we optimize this loop to be tail call optimized?

As the only thing this function does is pattern match on a list and Elixir also support pattern matching on function arguments we can rewrite this loop as several functions where the arguments vary as they did in the case statement.

```elixir
deltas = loop([], nil, measurements)

defp loop(acc, _, []), do: acc
defp loop(acc, nil, [head | tail]), do: loop(acc, head, tail)
defp loop(acc, prev, [head | tail]) do
  diff = (head.value - prev.value) / diff_string_date(prev.date, head.date)
  loop([diff | acc], head, tail)
end
```

You can see we only call ourselves as the last statement, yeah, we're tail call optimized and the compiler will take it from here.

Then all its left is calculating the average of the deltas and multiply it with the number of days from the last measurement to the 31st of December and we are there

```bash
~>./van_moofing trend 2020
With a average of 4.33 km a day and 13 days till the end of the year,
you will probably cycle another 56.0 km in 2020 for a grand total of 10163.0 km

```

So I did not reach my goal of 4000 km in 2020 (12.000 km total).
The fact that I worked from home since march due to Covid19 might have something to do with that.

BTW: I used the excellent [ex\_cli](https://hex.pm/packages/ex_cli) library for creating a command-line app, with argument parsing

The total application source code can be found here: [https://gitlab.com/gertjana/van\_moofings](https://gitlab.com/gertjana/van_moofings)
