在存储领域, 最常见的防止数据丢失的方式就是备份。家用 NAS 可能直接上 RAID ，而大数据领域的 HDFS 会默认存双副本。
副本越多提供的数据可靠性也越强，也意味着空间利用率越低，要达到对象存储 11个9 的数据可靠性，通常需要 4 副本，如此计算，真正的利用率只有 25%。

**纠删码则可以在提供和多副本相同的数据可靠性的同时，极大的提高空间利用率**。在 03 年的 GFS 初版论文中，Google 就曾设想利用纠删码来存储，然而受限于当时的技术发展无法如愿。十多年过去了，当时遥不可及的技术，如今却频繁的用于某些大中厂自研的系统中，颇有 *飞入寻常百姓家* 的感触。

MinIO 使用纠删码 (Erasure Code) 来存储数据，下文简称为 EC。EC 将**原始数据**均分为 $N$ 份，然后生成 $M$ 份相同大小的 **校验数据**，在这 $(N+M)$ 份数据中任意丢失 $M$ 份均可生成原始数据。 MinIO 确保这 $(N+M)$ 份数据分散在 $(N+M)$ 个不同的磁盘上，防止磁盘损毁导致数据丢失。常用的 $(N+M) = 8+4$，提供比 4 副本更强的数据可靠性，同时空间利用率维持在 75%。

EC 的原理是数学中的矩阵运算，设想存在一份原数据: *"abcdefghijklmnop"*，共 16Byte.

假设 $(N+M) = 4 + 2$，我们要将原数据切分为 4 份，用矩阵表示如下，每行代表一份数据


如果我们用另一个 $6 \times 4$ 矩阵和原始数据相乘:


$$
\begin{bmatrix}
   01 & 00 & 00 & 00 \\
   00 & 01 & 00 & 00 \\ 
   00 & 00 & 01 & 00 \\ 
   00 & 00 & 00 & 01 \\ 
   1b & 1c & 12 & 14 \\ 
   1c & 1b & 14 & 12
\end{bmatrix}
\times
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   i & j & k & l \\ 
   m & n & o & p \\ 
\end{bmatrix}
\=
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   i & j & k & l \\ 
   m & n & o & p \\ 
   51 & 52 & 53 & 49 \\ 
   55 & 56 & 57 & 25 \\ 
\end{bmatrix}
$$

这样就生成了两份新数据，假设我们丢失了两份数据 ~~*i j k l*~~ 和 ~~*m n o p*~~，现有数据变为

$$
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   51 & 52 & 53 & 49 \\ 
   55 & 56 & 57 & 25 \\ 
\end{bmatrix}
$$

因为

$$
\begin{bmatrix}
   01 & 00 & 00 & 00 \\
   00 & 01 & 00 & 00 \\ 
   1b & 1c & 12 & 14 \\ 
   1c & 1b & 14 & 12
\end{bmatrix}
\times
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   i & j & k & l \\ 
   m & n & o & p \\ 
\end{bmatrix}
\=
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   51 & 52 & 53 & 49 \\ 
   55 & 56 & 57 & 25 \\ 
\end{bmatrix}
$$

如果我们同时乘一个逆矩阵

$$
\begin{bmatrix}
   01 & 00 & 00 & 00 \\
   00 & 01 & 00 & 00 \\ 
   1b & 1c & 12 & 14 \\ 
   1c & 1b & 14 & 12
\end{bmatrix}^{-1}
\times
\begin{bmatrix}
   01 & 00 & 00 & 00 \\
   00 & 01 & 00 & 00 \\ 
   1b & 1c & 12 & 14 \\ 
   1c & 1b & 14 & 12
\end{bmatrix}
\times
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   i & j & k & l \\ 
   m & n & o & p \\ 
\end{bmatrix}
\ =
\begin{bmatrix}
   01 & 00 & 00 & 00 \\
   00 & 01 & 00 & 00 \\ 
   1b & 1c & 12 & 14 \\ 
   1c & 1b & 14 & 12
\end{bmatrix}^{-1}
\times
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   51 & 52 & 53 & 49 \\ 
   55 & 56 & 57 & 25 \\ 
\end{bmatrix}
$$

消去单元矩阵有

$$
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   i & j & k & l \\ 
   m & n & o & p \\ 
\end{bmatrix}
\=
\begin{bmatrix}
   01 & 00 & 00 & 00 \\
   00 & 01 & 00 & 00 \\ 
   1b & 1c & 12 & 14 \\ 
   1c & 1b & 14 & 12
\end{bmatrix}^{-1}
\times
\begin{bmatrix}
   a & b & c & d \\
   e & f & g & h \\ 
   51 & 52 & 53 & 49 \\ 
   55 & 56 & 57 & 25 \\ 
\end{bmatrix}
$$

如此，我们便可用丢失后的数据还原原有的数据。


可是这里仍然存在两个问题:
1. 如何知道对应数据的逆矩阵并且确保其存在性?
2. 如何保证矩阵相乘后每个单元的数据仍然只需要一字节存储?

## Vandermonde Matrix
假设 EC 采用 $N + M$，其中 $N$ 是原始数据， $M$ 是纠删数据，可以知道生成矩阵是 $(K+M) \times K$ 维的。 

对于矩阵前 $K$ 行是单位矩阵，而矩阵的后 $M$ 行则需要保证矩阵的任意 $k$ 行祖成的方阵都是可逆的。

工程化中利用了 Vandermonde Matrix，满足下面要求的则是一个 Vandermonde Matrix

$$
V = 
\begin{vmatrix}
1 & x_{0} & x_{0}^{2} & \dots & x_{0}^{n} \\
1 & x_{1} & x_{1}^{2} & \dots & x_{1}^{n} \\
1 & x_{2} & x_{2}^{2} & \dots & x_{2}^{n} \\
\vdots & \vdots & \vdots & \ddots &\vdots \\
1 & x_{n} & x_{n}^{2} & \dots & x_{n}^{n}
\end{vmatrix}
$$

**Vandermonde Matrix 大概率是可逆的!!**

而要求生成矩阵 


$$
V 
\= 
\begin{vmatrix}
01 & 00 & 00 & \dots & 00 \\ 
00 & 01 & 00 & \dots & 00 \\
00 & 00 & 01 & \dots & 00 \\
\vdots & \vdots & \vdots & \ddots &\vdots \\
k_0 & k_1 & k_2 & \dots & k_{k-1} \\
\vdots & \vdots & \vdots & \ddots & \vdots \\
\{k+m-1\}_0  & \{k+m-1\}_1 & \{k+m-1\}_2 & \dots & \{k+m-1\}_k-1 \\ 
\end{vmatrix} 
$$

可以根据 Vandermonde Matrix 得出

$$
V = 
\begin{vmatrix}
1 & x_{0} & x_{0}^{2} & \dots & x_{0}^{k-1} \\ 
1 & x_{1} & x_{1}^{2} & \dots & x_{1}^{k-1} \\
1 & x_{2} & x_{2}^{2} & \dots & x_{2}^{k-1} \\
\vdots & \vdots & \vdots & \ddots &\vdots \\
1 & x_{k+m-1} & x_{k+m-1}^{2} & \dots & x_{k+m-1}^{k-1}
\end{vmatrix}
\times
\begin{vmatrix}
1 & x_{0} & x_{0}^{2} & \dots & x_{0}^{k-1} \\ 
1 & x_{1} & x_{1}^{2} & \dots & x_{1}^{k-1} \\
1 & x_{2} & x_{2}^{2} & \dots & x_{2}^{k-1} \\
\vdots & \vdots & \vdots & \ddots &\vdots \\
1 & x_{k-1} & x_{k-1}^{2} & \dots & x_{k-1}^{k-1}
\end{vmatrix}^{-1}
$$

**此矩阵大概率是可逆的，不过好在 $$K + M$$ 在生产环境中往往是固定的几个可选值，可以提前确定生成矩阵。**

## Galois Field 

先了解什么是 *Field(域)*。

通常来说， Field 是一个**元素集合**，对于其中的**任意两个元素**都支持 $\oplus$ 和 $\otimes$ 两种操作，并且需要满足以下性质:

* 结合律: $a \oplus (b \oplus c) = (a \oplus b) \oplus c$; $a \otimes (b \otimes c ) = (a \otimes b) \otimes c$
* 交换律: $a \oplus b = b \oplus a$; $a \otimes b = b \otimes a$
* 分配律: $a \otimes (b \oplus c) = (a \otimes b) + (a \otimes c)$
* 存在 $\oplus$ 和 $\otimes$ 单元: 
   对于一个 Field *F*
   * $\exists a \in F, \forall b \in F, a \oplus b = b$
   * $\exists x \in F, \forall y \in F, x \otimes y = y$
* 存在 $\oplus$ 和 $\otimes$ 逆元:
   假设 *a* 和 *x* 分别是 $\oplus$ 和 $\otimes$ 单元，对于 Field *F*
   * $\forall b \in F, \exists c \in F，b \oplus c = a$
   * $\forall y \in F \land y \neq a, \exists z \in F，y \otimes z = x$

常见的 Field 如 *有理数域*, *实数域*，*复数域*，其中 $\oplus$ 和 $\otimes$分别对应 $+$ 和 $\times$， 这些域内元素个数都是无限的。而**具有有限个元素**的域就是*有限域*，又被称为*Galois Field*(纪念埃瓦里斯特·伽罗瓦)。

一个简单的有限域是 $(0, 1, 2)$，定义 $\oplus$ 和 $\otimes$ 如下: 结合上文中对域的定义，可以知道 $\oplus$ 和 $\otimes$ 是符合域的所有性质的。

$\oplus$|0  |1  |2  |──|$\otimes$|0  |1  |2  
--------|---|---|---|--|---------|---|---|---
0       |0  |1  |2  |──|0        |0  |0  |0  
1       |1  |2  |0  |──|1        |0  |1  |2  
2       |2  |0  |1  |──|2        |0  |2  |1  

有限域的符号表示为 $GF(q)$，其中 $q$ 代表有限域的元素个数，又称为*阶*，上文的有限域就是 $GF(3)$。**在 Erasure Code 中就需要使用 $GF(256)$ 的有限域**，因为需要将数限定在一个字节可表示的范围内。根据域的定义很难通过直觉直接构建出一个有限域，好在数学家们发明了一种可实践的有限域生成方法。

构建 $GF(256)$ 其实只需要填充下表，使 $\oplus$ 和 $\otimes$ 满足域的性质就可。

$\oplus$|0  |1  |...|255|──|$\otimes$|0  |1  |...|255
--------|---|---|---|---|--|---------|---|---|---|---
0       |   |   |   |   |──|         |   |   |...|   
1       |   |   |   |   |──|         |   |   |...|   
...     |   |   |   |   |──|         |   |   |...|   
255     |   |   |   |   |──|         |   |   |...|   

**容易想到，对于 $\oplus$ 只需做 *异或运算*，0 为 $\oplus$ 单元，并且容易证明满足交换律等各种性质。**

对于 $\otimes$ 运算，需要借助多项式乘法：
   1. 首先将元素映射为一个多项式。假设要计算 $23 \otimes 45$， 23 写成二进制的格式为 $0b10111$，转化为多项式就是 $x^4+x^2+x^1+x^0$，45 二进制格式为 $x^5+x^3+x^2+x^0$

   2. 执行多项式相乘 
   $$23 \otimes 45 \= (x^4+x^2+x^1+x^0) \times (x^5+x^3+x^2+x^0) \= x^9 + 2x^7 + x^6 + 2x^5 + 2x^4 + x^3 + 2x^2 + 1$$

   3. 将相乘结果对 $x^8+x^4+x^3+x^2+1$ 做 *特殊的取模* 运算。多项式取模采用 *长除法*，长除法后的模为 $2x^7+2x^6+x^5+2x^4+x^3+2x^2+1$。但是要对结果做进一步变换:
      * 将系数为负数的项式直接对系数取反。(这个例子中没负数，就不举例了)
      * 将系数为偶数的项式直接略去。
      * 将系数为奇数的项式系数变为1。处理后结果变为 $x^5+x^3+x^0$ 

   4. 将结果转换为二进制 $0b101001$，值为 *41*，因此 $23 \otimes 45 = 41$

这种 $\otimes$ 很容证明符合交换律，分配律和结合律, 而 *1* 就是乘法单元。
但是如何证明除 *0* 之外每个元素都有乘法逆元？

根据上图的 $mod$ 定义，
$$x^{113} \mod (x^8+x^4+x^3+x^2+1) \= (x^4+x^2+x^1+x^0) \mod (x^8+x^4+x^3+x^2+1)$$
因此认为在 $\otimes$ 的语义下， $x^{113} \= (x^4+x^2+x^1+x^0) = 23$。

对于集合$GF(256)$ $F$, 存在推论:
1. $$\forall~a \in F \land a \neq 0, \exists ~k \in F , x^k = a$$
2. $$x^{255} = x^0 = 1$$

因此元素 a 的乘法逆元就是 $x^{255 - log_{x}^{a}}$，存在且唯一。至于为什么有这两个推论...(证不出来睡大觉!)

并不是对于任何 $q$，都存在 $GF(q)$. 数学家已经证明只有当 $q = p^k \land p \in Prime \land k \in Positive$ 时存在。而 $x^8+x^4+x^3+x^2+1$ 又被称为 *本原多项式*，当 $q$ 满足上述条件时存在且不唯一，对于 $GF(256)$ 常用的就是这个多项式。

如果用代码按上述流程实现 $\otimes$，复杂度是很高的，首先是多项式相乘，然后取模。真正实践过程中，往往采用查表的方式。我们已经知道
$$\forall~a \in F \land a \neq 0, \exists ~k \in F , x^k = a$$
所以
$$a \otimes b \= x^{log_x^a} \times x^{log_x^b} \= x^{log_x^a+log_x^b}$$

因此我们只需知道 $\forall~k \in F$, $log_x^k$ 和 $x^k$ 的结果就可以很简单的计算 $\otimes$。
由于 $GF(256)$ 元素只有 256 个，完全可以通过打表的方式一一列出。

生产实践中往往有两张表， $exp$ 和 $log$ 分别表示取幂和取对数。这里由于 $GF(2^8)$ 元素太多，下面列出 $GF(2^4)$ 的表:

|k  |0 |1 |2 |3 |4 |5 |6 |7 |8 |9 |10|11|12|13|14|15|
|---|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
|exp|1 |2 |4 |8 |3 |6 |12|11|5 |10|7 |14|15|13|9 |12|
|log|- |0 |1 |4 |2 |8 |5 |10|3 |15|9 |6 |6 |13|11|2 |


构造出 $GF(256)$ 后将前文的矩阵乘法在此域内做计算，就可以保证任何结果运算皆用 1 字节存储。
