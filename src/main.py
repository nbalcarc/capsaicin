import Levenshtein as lv

import structs
import algorithms


def main():
    """Main entry point."""
    x = lv.distance("hi", "hello")
    print(x)



if __name__ == "__main__":
    main()


