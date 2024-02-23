import argparse




class WC:
    '''a basic re-implementation of the Unix wc command line tool in Python3.10'''

    parser = argparse.ArgumentParser(
        prog="ccwc",
        description="a clone of the Unix wc program",
        epilog="not production-ready!"

    )


    parser.add_argument(
        "-c",
        "--bytes",
        dest="bytes",
        help="display the number of bytes in the target file"

    )


    parser.add_argument(
    "-m",
    "--chars",
    dest="chars",
    help="display the number of characters in the target file"

    )

    parser.add_argument(
        "-l",
        "--lines",
        dest="newlines",
        help="display the number of newlines in the target file"

    )

    parser.add_argument(
    #this ought to be an interesting thing to implement in python3!
    "-",
    "--files0-from",
    dest="readfrom",
    help=" read  input  from  the files specified by NUL-terminated names in file F; If F is - then read names from standard input"
    )


    parser.add_argument(
        "-w",
        "--words",
        dest="words",
        help="display the number of words in the target file"

    )

    parser.add_argument(
    "-L",
    "--max-line-length",
    dest="maxlen",
    help="display the maximum display width"

    )



if __name__ =="__main__":



    def count_newlines(target):
        line_count = 0
        for line in target:
            if line.endswith("\n"):
                line_count += 1
        return line_count
    

    wc = WC()
    args = wc.parser.parse_args()

    if args.bytes:
        #read and print num of bytes, and target name to stdout
        try:
            import os, pathlib
            file_path = pathlib.Path(args.bytes)
            print(os.stat(file_path).st_size, file_path)
        except Exception as e:
            print(e)


    elif args.chars:
         #read and print num of characters and target name, to stdout

        try:
            from pathlib import Path
            file_path = Path(args.chars)
            with open(file_path, "r") as file:
                data = file.read()
            print(len(data), file_path)
        except Exception as e:
            print(e)
 

    elif args.newlines:        
        try:
            from pathlib import Path
            file_path = Path(args.newlines)
            with open(file_path, "r") as file:
                data = file.read()
                line_count = count_newlines(data)
            print(line_count, file_path)
        except Exception as e:
            print(e)


    elif args.readfrom:
        print(args.readfrom)
        try:
            if args.readfrom == "-" or None:
                import sys
                line_count = 0
                for line in sys.stdin:
                    while line is not None:
                        line_count += 1
                    else:
                        break
                print(line_count, args.readfrom)
            else:
                from pathlib import Path
                file_path = Path(args.readfrom)
                if not file_path.exists():
                    raise FileNotFoundError
                else:
                    with open(file_path, "r") as source_file:
                        sources = source_file.read().split("\\0")
                        for src in sources:
                            with open(src ,"r") as file_to_read:
                                data = file_to_read.read()
                                line_count = count_newlines(data)
                                import os 
                        print(len(data), os.stat(data).st_size)
        except Exception as e:
            print(e)
    elif args.words:
        print("num of words in the file")
    elif args.maxlen:
        print("max length of file contents")
   
   
     
