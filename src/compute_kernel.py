"""Pairwise Euclidean distances between turbine positions."""

from __future__ import annotations

import numpy as np


def turbine_pairwise_distances(x: np.ndarray, y: np.ndarray) -> np.ndarray:
    xs = np.asarray(x, dtype=float)
    ys = np.asarray(y, dtype=float)
    n = len(xs)
    out = np.zeros(n * n, dtype=float)
    for i in range(n):
        for j in range(n):
            dx = xs[i] - xs[j]
            dy = ys[i] - ys[j]
            out[i * n + j] = (dx * dx + dy * dy) ** 0.5
    return out
