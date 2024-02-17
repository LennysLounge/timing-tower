Number are treated as literals.
`0.0`, `12`, `3.1415`,  
with plus or minus unary  
`+12.1234`, `-0.657`

String literals is everything inside ".
`"Hello"` would producer -> `Text(String{"Hello"})`

Booleans are either `true` or `false`.

Simple math operations are:
`+`, `-`, `*`, `/`.

Numbers can be calculated together using the operations:  
`12.0 + 18` -> `Number(30.0)`  
`10 / 3.3` -> `Number(3.0303)`  
`5 * 6.0` -> `Number(30.0)`  
`10 - 8` -> `Number(2.0)`  


Values produced by producers can be referenced using brackets.
`[producer name]` -> whatever value the producer gives.

Do math with producers and numbers  
`[position] + 6`  
or with text  
`[driver name] + "dude"`


Call function by their name and `(arg1, arg2, ..)` for the arguments.  
`add( 12, 18 )` -> `Number(30)`  
`uppercase([driver name])` -> driver name but in upper case

Calculate the width of some text:  
`text_width([session time remaining])`


If then else:  
`if([is in pits], 20.0, - 20.0)`

Clamp value x between lower and upper  
`clamp([speed], 100, 200)`

Min and max:  
`min(12, 8)` -> `Number(8)`  
`max(12, 8)` -> `Number(12)`  

Map a value x that goes from x_low to x_high to low to high  
`map([speed], 0, 300, 50, 60)`







