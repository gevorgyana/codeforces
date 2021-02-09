/*
The string looks like this.
a ..
    .
    .
    b

2 ops:
 - take a
 - take b

is there always a solution? in theory, it can be the case that there is
no solution; it happens in the following case:

x | y

w1(x) > w1(y)
w2(x) > w2(y)

the weight of the upper part can only decrease,
the weight of the lower part can both decrease and increase.
the weight of the upper part is always greater than the weight of the
lower part, multiplied by a constant.

10 > 8 * 0.8

if the pearl from the lower part weights more than the pearl that would
slide down from the upper side, then we take the pearl from the
lower part, because that is more profitable than taking the leftmost
upper pearl (given constant cost of each pearl).

if the weigth is constant, then if the leftmost upper pearl costs more
than the lowest pearl, then we take it (if the weight of the upper part
remains greater than the critical notch). otherwise, we take the lowest
pearl and pull the string down by one pearl.

Does the order matter? Does it matter if we take one pearl off the lower
part, and then one off the upper part, and not vice versa? Let's say first
the cost is constant. Then we can take everything that we can from the
lower part, and then take whatever is left from the upper part.

[ translate from Russian ]

Надо попробовать принять некоторые величины за константы и подергать
другие.

Каждое действие будет сказываться на том, что верхняя жемчужина уходит из общего знака суммы для верхних жемчужин. Какая именно -- зависит от того, какую мы заберём: если заберём нижнюю, то уйдет верхняя крайняя правая; если заберём верхнюю, то уйдет она же.

То есть либо мы берём нижнюю и двигаем жемчужины на одну единицу вправо, либо мы берём верхнюю.

Допустим одинаковый вес жемчужин. Теперь покажем, что нет смысла жадно брать самые подходящие жемчужины. Допустим, есть. Но тогда может находиться такая жемчужина, что ее наличие даст нам оптимальное решение, но мы до нее не доберёмся, потому что будем брать жемчужины, которые расположены внизу, и таким образом оставим самую ценную жемчужину среди тех, которые останутся висеть и которые взять мы уже не сможем из-за перегрузки.

В том идеальном случае, когда вес одинаковый (и коэффициент трения равен единице), мы максимум сможем получить строку из жемчужин длиной в 2k ячеек на том этапе, когда забирать жемчужины мы уже не можем. То есть мы максимум возьмём (и на самом деле, при равном весе жемчужин, так и будет) n - 2k жемчужин.

What should be the rule that we must obey so that we prune the 'bad'
branches in our search tree? We can't just run a complete search of the
actions, because if we did so, we would have 2 ^ 1000 options to choose
from.

Whatevers hangs, will remain hanging, except for the lowest node, which
can be picked at the next round.

At any point, the upper part of the string is a subarray of the one that
was initially the upper part of the string. We can represent any state
with 2 integers. If we pick the leftmost node, we shift the leftmost
index by one. Otherwise, we shift the rightmost index to the left by one.

So one more time, we can remove either the leftmost or the rightmost node
(if we are talking about the upper part of the string). The cost of
doing the former is decreasing the balance by some term T, the gain of
doing so is cost[leftmost node]. The cost of doing the latter is
decreasing the balance by diff(weight[rightmost node],
weight[lowest node]), the gain is cost[lowest node - i], where i is the
number of nodes that we have taken from the lower part of the string.

So if we cache everything, we get O(1000) * O(1000) of time complexity.
It is enough to pass the tests (hopefully).
*/

use std::io::{Read, Stdin, self};
use concise_scanf_like_input::read_input_lines;
use concise_scanf_like_input::read_line_of_vars;

fn main() {
    let mut buf = String::new();
    let mut lines: &[&str] = read_input_lines!(buf);
    let (n, m, k): (i32, i32, i32);
    read_line_of_vars!(i32, lines, n, m, k);
    let mut we: Vec<i32> = Vec::with_capacity(n as usize);
    let mut co: Vec<i32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
	let (w, c): (i32, i32);
	read_line_of_vars!(i32, lines, w, c);
	we.push(w);
	co.push(c);
    }
    println!("{:?}", we);
    println!("{:?}", co);

    // not ready yet
}
