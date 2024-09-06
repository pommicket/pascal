# Repeated numbers in Pascal's Triangle

Every number ${n \choose k}, 1<k<n-1$ appears at least four times in Pascal's triangle:

$${n\choose k} = {n\choose n-k} = {{n\choose k}\choose 1}={{n\choose k}\choose {n\choose k}}$$

This gives rise to the natural question of which numbers appear more than four times. Or, taking
out the symmetry and trivial case of ${n\choose 1}$, what are the solutions to
$${n\choose k}={m\choose l},$$
for $1<k\leq \frac n2$, $k<l\leq \frac m2$.

According to ["A Note on the Diophantine equation (x 4)=(y 2)"](https://www.researchgate.net/publication/235418296_A_Note_on_the_Diophantine_equation_x_4y_2)
(Ákos Pintér, 1995),
the cases of $(k,l)=(2,3)$ and $(k,l)=(2,4)$ have been solved completely,
and from ["Equal binomial coefficients: some elementary considerations"](https://repub.eur.nl/pub/1356/1356_ps.pdf)
(Benjamin M.M. de Weger, 1995), we also know that there are no solutions with $(k,l)=(3,4)$; so here is the full list of
solutions with $l \leq 4$.

<table>
<tr><th>k</th><th>l</th><th>n</th><th>m</th><th>n choose k = m choose l</th></tr>
<tr><td>2</td><td>3</td><td>16</td><td>10</td><td>120</td></tr>
<tr><td>2</td><td>3</td><td>56</td><td>22</td><td>1540</td></tr>
<tr><td>2</td><td>3</td><td>120</td><td>36</td><td>7140</td></tr>
<tr><td>2</td><td>4</td><td>21</td><td>10</td><td>210</td></tr>
</table>

Also an infinite family of other solutions is known

$${F_{2i+2} F_{2i+3}\choose F_{2i}F_{2i+3}} = {F_{2i+2} F_{2i+3} -1 \choose F_{2i}F_{2i+3} +1}$$

where $F_i$ is the $i$th Fibonacci number.

De Weger did a computer search up to ${n\choose k} < 10^{30}$ in 1995. Now that we have
faster computers we can go higher — and we can say for certain that the only solutions
up to ${n\choose k}<10^{42}$ are the ones found by de Weger: namely, the ones in the infinite
family above,

$${15 \choose 5} = {14 \choose 6} = 3003 = {78 \choose 2}$$

$${104 \choose 39} = {103 \choose 40} = 61218182743304701891431482520$$

And the “sporadic” solutions:

$$ {153 \choose 2} ={19\choose 5} = 11628$$

$$ {221\choose 2}={17 \choose 8}=24310$$

This program searches up to ${n\choose k} <10^X$, when given the arguments `entry-limit X`.
The search works by putting all ${n\choose k}<10^X$ with $5\leq k\leq \frac n 2$ into
an array and sorting it. Then we check for adjacent elements which are equal,
and use a binary search to check whether elements in the array are ${n\choose 2},{n\choose 3},{n\choose 4}$.
This finds all solutions with $l>4$ (since the solutions with $l\leq 4$ are known).

We can make a slight optimization by just storing the entries mod $2^{64}$ in the array,
then only doing the full comparison on entries who agree mod $2^{64}$.
But still searching to $10^{42}$ with this method already requires 11 GB of memory.

Using this modular trick we can also search up to the 60,000th row (use arguments `row-limit 60000`),
and (sadly) confirm that the only repeated entries are the ones listed above and the new entries in the infinite family,

$${713\choose 273} = {714\choose 272} \approx 3.5 \times 10^{204}$$

$${4894\choose 1870} = {4895\choose 1869} \approx 4.6 \times 10^{1141}$$

$${33551 \choose 12816} = {33552 \choose 12815} \approx 6.0 \times 10^{9687}$$

Again we run into a memory bottleneck — searching this far required 14 GB of memory.

We can sidestep the large memory requirements almost entirely if we assume that sporadic solutions have
very small values of $k$ (this is motivated by the two known sporadic solutions having $k=2$). Then we just
go through each entry in the triangle, only storing one row in memory at a time, and do a binary search to check if
the entry is ${n\choose k}$ for some small $k$. Using this method we can verify quickly and with just a few megabytes of memory
that no more solutions exist with ${n\choose k}<10^{64}, n,m < 3\times 10^6, k \leq 10$ (arguments `col-limit 10`).
