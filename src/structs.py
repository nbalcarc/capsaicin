from __future__ import annotations


class TreeNode:
    """One node in the phylogenetic tree."""
    label: str  #contains kingdom, phylum, class name
    threshold: float
    children: list[TreeNode]
    type_species: Genome


class Genome:
    """Contains all info required for a genome."""
    label: str
    file_path: str





