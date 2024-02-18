from itertools import chain


K = 32


def generate_kmers(line: str) -> list[str]:
    """Generate kmers for a given string."""
    global K
    if len(line) < K: #return nothing if line is too small
        return []
    kmers = [""] * (len(line) - K + 1) #initialize list size
    for i in range(len(kmers)): #generate all kmers
        kmers[i] = line[i:i+K]
    return kmers


def kmers_from_file(path: str) -> set[str]:
    """Generate kmers from a genome file."""
    with open(path, "r") as file:
        lines = file.readlines()
    kmers_raw = map(generate_kmers, filter(lambda x: not x.startswith(">"), lines))
    kmers = set(chain.from_iterable(kmers_raw)) #flatten all kmers
    return kmers


