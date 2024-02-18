import Levenshtein as lv

import structs
import algorithms


def main():
    """Main entry point."""
    x = lv.distance("hi", "hello")
    print(x)
    kmers = algorithms.kmers_from_file("/run/host/run/media/terrior/BeutelratteDrive/Genomes/isolated/droseraCapensis.fna")


if __name__ == "__main__":
    main()


