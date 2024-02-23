
# Things (Re)Discovered


## Errors

+ Irrefutable pattern is allowed only for the last case 

This error occurred while trying to use `match` statements to handle the options in the arguments within the return of the `parse_args()` function of the Argparse module, and led to a deeper look at how match statements work in Python. Originally, I attempted to use the `match` statement like this:
    <code>
    if (dunder)__name__=="__main__":
        args = parser.parse_args()

         match args:
            case word:
                print("reading number of words in file")
                file = "text.txt"
                with open(file, "r") as f:
                    data = f.read()
                    .....
            case chars:
                pass

And found that the first match threw the error mentioned above. 

#### revisiting the simple `match` Tutorial

For simplicity, there are two patterns in a match statement, the subject pattern, which appears immediately after the `match` keyword, and the comparison pattern, which appears after the `case` keyword. Capture patterns are variables within the comparison pattern, literal patterns are ints, floats, strings etc within the same, and wildcards are catch-alls using the underscore.

+ The `match` statement evaluates the subject pattern against the comparison pattern's structure, only regarding the occasions when both patterns are identical as a successful match. It also binds the names in the components of the comparison pattern to those of the subject pattern. Eg `match tuple(a,b): case [x,y]:` will bind the values in `list(x,y)` to the pattern in `tuple(a,b)`.
+ When it isn't immediately clear how many subjects there will be in a match statement (for instance, when reading from user input or accepting arguments from the command line or over a network), the extended unpacking syntax can be used to assign multiple values found to a sequence within the comparison pattern. Eg: `match sys.argv[2]: case ["countwords", *filenames]:` where there could be multiple files intended to be processed in sequence or asynchrounously.
+ Python uses the underscore `_` as a catch-all for patterns which the programmer does not care to match against. Eg `match something: case _: print("unrecognised pattern")`. This is called a __wildcard__, not to be confused with the asterix, which has the same name in other domains.
+ __Or patterns__ can be used to match against more than one comparison pattern, or pattern containing a sequence (sequence pattern) using the pipe operator `|` to separate the options. This is useful for patterns which can appear in different ways but mean the same thing, such as the Unix `-h, -H, --help` command line flags.
+ __As patterns__ allow the capture (into variables) of sub-patterns using the syntax: `case ["parse_as", ("json" | "yaml" | "html")] as output: result = parse_as(output)` where there are multiple options available for the expected kind out of output.
+ __Guards__ are conditionals used within `case` comparisons as a final means of ascertaining the validity of the comparison pattern. They are implemented using `if` keywords. Eg: `case ["do_action", target] if target in accepted_targets` where the `accepted_targets` capture pattern is a previously established sequence of acceptable targets to carry out this operation on.

Okay, so capture patters cant be used more than once in Python match statements? Looks like I know what I'll spend this weekend researching! Knowing the Python folks, there's a good reason for this, and I'm probably spoiled from writing strongly typed languages and showering myself in all that Rust goodness.






## enumerate(item) vs len(item)

Is python's enumerate function zero indexed? Else, why is there a consistent discrepancy between what it returns and what the len() function returns when comparing the same strings?


## Error handling and appropriate tests and error statements on each function call, aka the Todos.

+ tests and appropriate errors for nonexistent files
+ tests and appropriate errors for corrupted or non-utf8 files
+ errors for non-files and non-string values
